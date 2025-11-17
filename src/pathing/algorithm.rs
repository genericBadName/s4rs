use crate::config::Configuration;
use crate::pathing::action::{Moveset, SpatialAction};
use crate::pathing::data::{BinaryHeapOpenSet, Node, PathNode};
use crate::pathing::world::Space;
use dashmap::DashMap;
use std::hash::Hash;
use std::ops::Add;
use std::time::Instant;
use dashmap::mapref::one::RefMut;
use eyre::{eyre, OptionExt, Result};

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
    // TODO: Using only the position may not be unique enough, especially for moves that have overlapping locations (jumping/walking/etc.)
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
        let start_node = Node::start_node(start, &end);

        self.closed_set.insert(SpatialAction::new_root(start), start_node);
        self.open_set.insert(start_node)?;

        // run until all nodes are considered or time is up
        while !self.open_set.is_empty() && start_time.elapsed().le(&self.config.timeout) {
            if let Ok(c_node) = self.open_set.pop() {
                // if at the end, exit early
                if c_node.action.pos == end {
                    return self.retrace(c_node);
                }
                // otherwise run an iteration and update
                self.update_positions(c_node, &end)?;
            }
        }

        // no path found, return an empty one
        Ok(vec![])
    }

    /// Updates node positions and advances the pathfinding algorithm.
    fn update_positions(&mut self, current: Node<P>, end: &P) -> Result<()> {
        // moves determine which neighbor to check
        for action in self.moves.iter() {
            let neighbor_pos = current.action.pos + action.offset;
            if !self.space.can_move_to(neighbor_pos) { continue }

            let hazard = 1.0;
            //let hazard = get_hazard(neighbor_pos) as f64;
            let tentative_g_cost = (
                current.g_cost + (action.cost as f64) + current.action.pos.distance_to(&neighbor_pos)
            ) * hazard;

            // handle the different types of references
            let action_to = SpatialAction::new(neighbor_pos, *action);
            let mut neighbor = self.get_node_at(&action_to, &current)
                .ok_or_eyre("Couldn't get a node from the Closed Set")?;
            // if this neighbor is better than the current, add it to the open set
            if neighbor.g_cost - tentative_g_cost > MINIMUM_IMPROVEMENT {
                neighbor.parent = Some(current.heap_idx.unwrap());
                neighbor.g_cost = tentative_g_cost;
                neighbor.h_cost = neighbor_pos.distance_to(end);

                // don't have to update the closed set because the reference is mutable
                // but we do have to update the open set with a decrease-key operation
                if neighbor.is_open() {
                    self.open_set.sift_up(&neighbor)?;
                } else {
                    self.open_set.insert(neighbor)?;
                }
            }
        }

        Ok(())
    }

    /// Tries to get the node at the specified position `pos`. Will only return nodes at the frontier,
    /// and not nodes which have already been considered (closed).
    fn get_node_at(&'_ self, sa: &SpatialAction<P>, current: &Node<P>)-> Option<Node<P>> {
        if !self.closed_set.contains_key(sa) {
            // create a new closed node if one doesn't exist at that position.
            // its index will be incremented by 1.
            let new_closed = Node {
                g_cost: self.config.cost_inf,
                h_cost: 0.0,
                parent: current.heap_idx,
                action: *sa,
                heap_idx: None,
            };
            self.closed_set.insert(*sa, new_closed);
        }
        Some(*self.closed_set.get(sa)?.value())
    }

    /// Retraces the `Node` relationships to find the optimal path.
    fn retrace(&self, end: Node<P>) -> Result<Vec<PathNode<P>>> {
        let mut path: Vec<PathNode<P>> = Vec::new();
        let mut next: Option<usize> = end.parent;

        while let Some(current) = next {
            let current_node= self.open_set.get(current)
                .ok_or_else(|| eyre!("Node had a dangling parent (index {} did not exist)", current))?;
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
pub trait GraphPosition: Copy + Hash + Eq + Add<Output = Self> {
    /// Calculates the distance from one `NodePosition` position to another.
    fn distance(a: &Self, b: &Self) -> f64;
    /// Calculates the distance from `self` to another `NodePosition`.
    fn distance_to(&self, other: &Self) -> f64 {
        Self::distance(self, other)
    }
}