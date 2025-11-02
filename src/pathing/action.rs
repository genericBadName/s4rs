use crate::pathing::math::Vector3i;
use crate::vec3i;

/// A move action that can be taken by the pathfinding entity. These are the "lines" that
/// connect nodes on the graph.
#[derive(Debug, Copy, Clone)]
pub struct MoveAction {
    /// Initial cost to execute this move action.
    pub cost: f64,
    /// Offset from the current position to check for this action (the neighbor position).
    pub offset: Vector3i
}

pub fn default_moveset() -> Vec<MoveAction> {
    vec![
        MoveAction { cost: 1.0, offset: vec3i!(1, 0, 0) },
        MoveAction { cost: 1.0, offset: vec3i!(-1, 0, 0) },
        MoveAction { cost: 1.0, offset: vec3i!(0, 0, 1) },
        MoveAction { cost: 1.0, offset: vec3i!(0, 0, -1) },
    ]
}