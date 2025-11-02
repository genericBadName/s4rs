use crate::pathing::math::Vector3i;

/// A move action that can be taken by the pathfinding entity. These are the "lines" that
/// connect nodes on the graph.
#[derive(Debug, Copy, Clone)]
pub struct MoveAction {
    /// Initial cost to execute this move action.
    pub cost: f64,
    /// Offset from the current position to check for this action (the neighbor position).
    pub offset: Vector3i
}