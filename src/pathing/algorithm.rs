use crate::config::Configuration;
use crate::pathing::action::MoveAction;
use crate::pathing::data::{Node, PathNode};
use crate::pathing::world::Space;
use dashmap::DashMap;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::ops::{Add, Deref};
use std::rc::Rc;
use std::time::Instant;

const MINIMUM_IMPROVEMENT: f64 = 0.01;

/// Temporary state for A* path calculations. The type `P` is used as the "position in space" when
/// finding nodes. Any kind of cheap and easily copyable data can be used, as the A* algorithm does
/// not care for dimensionality, only relationships between nodes.
pub struct PathCalculator<P, S> where P: GraphPosition, S: Space<P> {
    /// The Open set (min-heap) involved in the calculation. These are nodes that have not been
    /// considered yet, but are known (the "node frontier").
    open_set: BinaryHeap<Reverse<Node<P>>>,
    /// The Closed set involved in the calculation. These are nodes that have already been
    /// considered and do not need to be re-examined.
    closed_set: DashMap<P, Node<P>>,
    moves: Rc<Vec<MoveAction<P>>>,
    space: Rc<S>,
    /// Configuration values for pathfinding.
    config: Rc<Configuration>
}

impl <P, S> PathCalculator<P, S> where P: GraphPosition, S: Space<P> {
    pub fn new(moves: Rc<Vec<MoveAction<P>>>, config: Rc<Configuration>, space: Rc<S>) -> PathCalculator<P, S> {
        PathCalculator {
            open_set: BinaryHeap::new(),
            closed_set: DashMap::new(),
            moves,
            space,
            config
        }
    }

    /// Calculates the optimal path from a start position to an end position.
    pub fn calculate(&mut self, start: P, end: P) -> Option<Vec<PathNode<P>>> {
        let start_time = Instant::now();
        let start_node = Node::start_node(start, &end);

        self.open_set.push(Reverse(start_node));

        // run until all nodes are considered or time is up
        while !self.open_set.is_empty() && start_time.elapsed().le(&self.config.timeout) {
            let current = self.open_set.pop();
            match current {
                None => break,
                Some(c_node) => {
                    // if at the end, exit early
                    if c_node.0.pos == end {
                        return Some(self.retrace(c_node.0));
                    }
                    // otherwise just run once
                    let curr_ref = Rc::new(c_node.0);
                    self.update_positions(curr_ref, &end);
                }
            }
        }

        // no path found, you get nothing.
        None
    }

    /// Updates node positions and advances the pathfinding algorithm.
    fn update_positions(&mut self, current: Rc<Node<P>>, end: &P) {
        // moves determine which neighbor to check
        for action in self.moves.clone().iter() {
            let neighbor_pos = current.pos + action.offset;
            if !self.space.can_move_to(neighbor_pos) { continue }

            let hazard = 1.0;
            //let hazard = get_hazard(neighbor_pos) as f64;
            let tentative_g_cost = (current.g_cost + action.cost) * hazard;

            // handle the different types of references
            let mut neighbor = self.get_node_at_pos(neighbor_pos, current.clone());
            // if this neighbor is better than the current, add it to the open set
            if neighbor.g_cost - tentative_g_cost > MINIMUM_IMPROVEMENT {
                neighbor.parent = Some(current.clone());
                neighbor.g_cost = tentative_g_cost;
                neighbor.h_cost = neighbor_pos.heuristic_from_self(end);

                self.open_set.push(Reverse(neighbor));
            }
        }
    }

    /// Tries to get the node at the specified position `pos`. Will only return nodes at the frontier,
    /// and not nodes which have already been considered (closed).
    fn get_node_at_pos(&mut self, pos: P, current: Rc<Node<P>>) -> Node<P> {
        if self.closed_set.contains_key(&pos) {
            // return a closed set node to guarantee path optimality
            self.closed_set.remove(&pos).unwrap().1
        } else {
            // otherwise, return a new node
            Node {
                g_cost: self.config.cost_inf,
                h_cost: 0.0,
                parent: Some(current),
                pos
            }
        }
    }

    /// Retraces the `Node` relationships to find the optimal path.
    fn retrace(&self, end: Node<P>) -> Vec<PathNode<P>> {
        let mut path: Vec<PathNode<P>> = Vec::new();
        let mut next: Option<Rc<Node<P>>> = Some(Rc::from(end));

        while let Some(current) = next {
            path.push(PathNode::from(current.deref()));
            next = current.parent.clone();
        }

        path.reverse();

        path
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
    /// Calculates the heuristic of one `NodePosition` position to another.
    fn heuristic(a: &Self, b: &Self) -> f64;
    fn heuristic_from_self(&self, other: &Self) -> f64 {
        Self::heuristic(self, other)
    }
}