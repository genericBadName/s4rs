use std::fmt::Debug;
use crate::config::Configuration;
use crate::pathing::action::{Moveset, SpatialAction};
use crate::pathing::data::{BinaryHeapOpenSet, Node, PathNode};
use crate::pathing::world::Space;
use dashmap::DashMap;
use std::hash::Hash;
use std::ops::Add;
use std::time::Instant;
use eyre::{eyre, OptionExt, Result};
use log::warn;

const MINIMUM_IMPROVEMENT: f64 = 0.01;

/// Temporary state for A* path calculations. The type `P` is used as the "position in space" when
/// finding nodes. Any kind of cheap and easily copyable data can be used, as the A* algorithm does
/// not care for dimensionality, only relationships between nodes.
pub struct PathCalculator<P, S> where P: GraphPosition, S: Space<P> {
    /// The Open set (min-heap) involved in the calculation. These are nodes that have not been
    /// considered yet, but are known (the "node frontier").
    open_set: BinaryHeapOpenSet<P>,
    /// The Closed set involved in the calculation. These are nodes that have already been
    /// considered and do not need to be re-examined.
    closed_set: DashMap<SpatialAction<P>, Node<P>>,
    /// The pathfinder's allowed moves.
    moves: Moveset<P>,
    /// The `Space` that this pathfinder will sample from.
    space: Box<S>,
    /// General configuration values for the pathfinding system.
    config: Configuration
}

impl <P, S> PathCalculator<P, S> where P: GraphPosition, S: Space<P> {
    pub fn new(moves: Moveset<P>, config: Configuration, space: Box<S>) -> PathCalculator<P, S> {
        // TODO: make index bound equal to the maximum amount of Voxels within a Chunk/region of Chunks
        PathCalculator {
            open_set: BinaryHeapOpenSet::new(),
            closed_set: DashMap::new(),
            moves,
            space,
            config
        }
    }

    /// Calculates the optimal path from a start position to an end position.
    pub fn calculate(&mut self, start: P, end: P) -> Result<Vec<PathNode<P>>> {
        let start_time = Instant::now();
        let mut start_node = Node::start_node(start, &end);

        self.closed_set.insert(SpatialAction::new_root(start), start_node);
        self.open_set.insert(&mut start_node)?;

        // run until all nodes are considered or time is up
        while !self.open_set.is_empty() {
            if let Some(c_node) = self.open_set.pop() {
                // if at the end, exit early
                if c_node.action.pos == end {
                    return self.retrace(c_node);
                }
                // otherwise run an iteration and update
                self.update_positions(c_node, &end)?;
            }

            if start_time.elapsed().ge(&self.config.timeout) {
                warn!("Pathfinder timed out.");
                //break
            }
        }

        // no path found, return an empty one
        Ok(vec![])
    }

    /// Updates node positions and advances the pathfinding algorithm.
    fn update_positions(&mut self, current: Node<P>, end: &P) -> Result<()> {
        println!("CURRENT: {:?}", current.action.pos);
        // moves determine which neighbor to check
        for action in self.moves.iter() {
            let neighbor_pos = current.action.pos + action.offset;

            let material_cost = self.space.material_cost(neighbor_pos);
            let tentative_g_cost = current.g_cost + (action.cost as f64) + material_cost;

            // handle the different types of references
            let action_to = SpatialAction::new(neighbor_pos, *action);
            let mut neighbor = self.get_node_at(&action_to, &current, end)
                .ok_or_eyre("Couldn't get a node from the Closed Set")?;
            // if this neighbor is better than the current, add it to the open set
            if neighbor.g_cost - tentative_g_cost > MINIMUM_IMPROVEMENT {
                neighbor.parent = Some(current.action);
                neighbor.g_cost = tentative_g_cost;
                neighbor.h_cost = neighbor_pos.distance_to(end);

                println!("neighbor: {:?}: {}", neighbor_pos, neighbor.f_cost());

                // decrease key of the open set
                if neighbor.is_open() {
                    self.open_set.sift_up(&mut neighbor)?;
                } else {
                    self.open_set.insert(&mut neighbor)?;
                }
            }

            // update parent
            self.closed_set.insert(action_to, neighbor);
        }

        Ok(())
    }

    /// Tries to get the node at the specified position `pos`. Depending on the cost of this node,
    /// it may be added to the Open Set or stay Closed.
    fn get_node_at(&self, action: &SpatialAction<P>, current: &Node<P>, end: &P) -> Option<Node<P>> {
        if !self.closed_set.contains_key(action) {
            // create a new closed node if one doesn't exist at that position.
            let new_closed = Node {
                g_cost: self.config.cost_inf,
                h_cost: 0.0,
                parent: Some(current.action),
                action: *action,
                heap_idx: None,
            };
            self.closed_set.insert(*action, new_closed);
        }

        Some(*self.closed_set.get(action)?.value())
    }

    /// Retraces the `Node` relationships to find the optimal path.
    fn retrace(&self, end: Node<P>) -> Result<Vec<PathNode<P>>> {
        let mut path: Vec<PathNode<P>> = Vec::new();
        path.push(PathNode::new(end.action));
        let mut next: Option<SpatialAction<P>> = end.parent;

        while let Some(current) = next {
            let current_node= self.closed_set.get(&current)
                .ok_or_eyre("Node had a dangling parent (SpatialAction did not exist)")?;
            path.push(PathNode::new(current_node.action));
            next = current_node.parent;
        }

        path.reverse();
        Ok(path)
    }

    /// Resets the calculator for reuse. Not strictly necessary, this is mainly for continually
    /// calculated on the same entity and preserving its moveset/config.
    pub fn reset(&mut self) {
        self.open_set.clear();
        self.closed_set.clear();
    }
}

/// Represents a point in graph space. It is a requirement that any position in graph space be
/// related to any other graph position in an N-dimensional space.
pub trait GraphPosition: Copy + Hash + Eq + Debug + Add<Output = Self> {
    /// Calculates the distance from one `NodePosition` position to another.
    fn distance(a: &Self, b: &Self) -> f64;
    /// Calculates the distance from `self` to another `NodePosition`.
    fn distance_to(&self, other: &Self) -> f64 {
        Self::distance(self, other)
    }
}