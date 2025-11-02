use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Deref;
use std::time::Instant;
use dashmap::DashMap;
use crate::config::Configuration;
use crate::pathing::action::MoveAction;
use crate::pathing::data::{Node, PathNode};
use crate::pathing::math::Vector3i;
use crate::pathing::world::get_hazard;

const MINIMUM_IMPROVEMENT: f64 = 0.01;

/// Temporary state for A* path calculations.
struct PathCalculator<'a> {
    /// The Open set (min-heap) involved in the calculation. These are nodes that have not been
    /// considered yet, but are known (the "node frontier").
    open_set: BinaryHeap<Reverse<Box<Node>>>,
    /// The Closed set involved in the calculation. These are nodes that have already been
    /// considered and do not need to be re-examined.
    closed_set: DashMap<Vector3i, Box<Node>>,
    moves: Vec<MoveAction>,
    /// Configuration values for pathfinding.
    config: &'a Configuration
}

impl PathCalculator<'_> {
    fn new(moves: Vec<MoveAction>, config: &'_ Configuration) -> PathCalculator<'_> {
        PathCalculator {
            open_set: BinaryHeap::new(),
            closed_set: DashMap::new(),
            moves,
            config,
        }
    }

    /// Calculates the optimal path from a start position to an end position.
    pub fn calculate(&mut self, start: Vector3i, end: Vector3i) -> Option<Vec<PathNode>> {
        let start_time = Instant::now();
        let start_node = Box::new(Node::start_node(start, &end));

        self.open_set.push(Reverse(start_node.clone()));
        self.closed_set.insert(start, start_node);

        // run until all nodes are considered or time is up
        while !self.open_set.is_empty() && start_time.elapsed().le(&self.config.timeout) {
            let current = self.open_set.pop();

            match current {
                None => break,
                Some(c_node) => {
                    // if at the end, exit early
                    if c_node.0.pos == end {
                        return Some(self.retrace(*c_node.0));
                    }
                    // otherwise just run once
                    self.update_positions(c_node.0, &end);
                }
            }
        }

        // no path found, you get nothing.
        None
    }

    /// Updates node positions and advances the pathfinding algorithm.
    fn update_positions(&mut self, current: Box<Node>, end: &Vector3i) {
        for action in self.moves.iter() {
            let neighbor_pos = Vector3i::offset(&current.pos, &action.offset);
            if !self.can_move_to(neighbor_pos) { continue }

            let mut neighbor_node = self.get_node_at_pos(neighbor_pos, &current);
            let hazard = get_hazard(neighbor_pos) as f64;
            let tentative_g_cost = (current.g_cost + action.cost) * hazard;

            // is this a better path?
            if neighbor_node.g_cost - tentative_g_cost > MINIMUM_IMPROVEMENT {
                neighbor_node.parent = Some(current.clone());
                neighbor_node.g_cost = tentative_g_cost;
                neighbor_node.h_cost = neighbor_pos.distance_squared(end);

                // if this was the lowest node, add it to the open set
                self.open_set.push(Reverse(neighbor_node));
            }
        }
    }

    fn get_node_at_pos(&self, pos: Vector3i, current: &Box<Node>) -> Box<Node> {
        match self.closed_set.get(&pos) {
            None => {
                let new_node = Box::new(Node {
                    g_cost: self.config.cost_inf,
                    h_cost: 0.0,
                    parent: Some(current.clone()),
                    pos
                });
                self.closed_set.insert(pos, new_node.clone());

                new_node
            },
            Some(existing_node) => {
                existing_node.value().clone()
            }
        }
    }

    /// Checks if the pathfinding entity can move to the specified position.
    fn can_move_to(&self, pos: Vector3i) -> bool {
        todo!()
    }

    /// Retraces the `Node` relationships to find the optimal path.
    fn retrace(&self, end: Node) -> Vec<PathNode> {
        let mut path: Vec<PathNode> = Vec::new();
        let mut next: Option<Box<Node>> = Some(Box::from(end));

        while let Some(current) = next {
            path.push(PathNode::from(current.deref()));
            next = current.parent;
        }

        path
    }
}