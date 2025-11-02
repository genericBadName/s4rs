use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::pathing::math::Vector3i;

/// A node within the A* graph.
#[derive(Debug, Clone)]
pub struct Node {
    /// Initial cost to traverse to this node.
    pub g_cost: f64,
    /// Heuristic cost to traverse to this node.
    pub h_cost: f64,
    /// Parent of this node. If `Option::None`, this is considered the root node.
    pub parent: Option<Box<Node>>,
    /// The node's position in space.
    pub pos: Vector3i
}
impl Node {
    pub fn start_node(start: Vector3i, end: &Vector3i) -> Node {
        Node {
            g_cost: 0.0,
            h_cost: start.distance_squared(&end),
            parent: None,
            pos: start
        }
    }
    
    pub fn f_cost(&self) -> f64 {
        self.g_cost + self.h_cost
    }
}

impl <'a> PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost().eq(&other.f_cost())
    }
}

impl <'a> PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.f_cost().partial_cmp(&other.f_cost())
    }

    fn lt(&self, other: &Self) -> bool {
        self.f_cost().lt(&other.f_cost())
    }

    fn le(&self, other: &Self) -> bool {
        self.f_cost().le(&other.f_cost())
    }

    fn gt(&self, other: &Self) -> bool {
        self.f_cost().gt(&other.f_cost())
    }

    fn ge(&self, other: &Self) -> bool {
        self.f_cost().ge(&other.f_cost())
    }
}

impl Eq for Node {}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_cost().total_cmp(&other.f_cost())
    }
}

/// A node on the path to a given destination. Does not contain unnecessary cost data like with
/// `Node`, only movement-related information.
#[derive(Debug, Copy, Clone)]
pub struct PathNode {
    pub(crate) pos: Vector3i
}

impl From<Node> for PathNode {
    fn from(value: Node) -> Self {
        PathNode {
            pos: value.pos
        }
    }
}

impl From<&Node> for PathNode {
    fn from(value: &Node) -> Self {
        PathNode {
            pos: value.pos
        }
    }
}

/// Multipliers for potential hazards in the pathing entity's way.
/// All methods take an `i32`, these are just explicitly defined for convenience.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct HazardMultiplier {
    /// Unknown. Could be any type of object.
    pub unknown: u32,
    /// Non-solid obstacle that the entity can pass through.
    pub non_solid: u32,
    /// Solid obstacle that the entity can't pass through.
    pub solid: u32,
    /// Dangerous obstacle that should be avoided.
    pub dangerous: u32
}

impl HazardMultiplier {
    /// Creates an instance with the default values.
    pub fn new() -> HazardMultiplier {
        HazardMultiplier {
            unknown: 10,
            non_solid: 21,
            solid: 10,
            dangerous: 50
        }
    }
}