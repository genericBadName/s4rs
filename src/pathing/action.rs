use crate::pathing::math::{Vector2i, Vector3i};
use crate::{vec2i, vec3i};
use std::ops::Add;

/// A move action that can be taken by the pathfinding entity. These are the "lines" that
/// connect nodes on the graph.
#[derive(Debug, Copy, Clone)]
pub struct MoveAction<P> where P: Copy + Add<Output = P> {
    /// Initial cost to execute this move action.
    pub cost: f64,
    /// Offset from the current position to check for this action (the neighbor position).
    pub offset: P
}

pub fn default_moveset() -> Vec<MoveAction<Vector3i>> {
    vec![
        MoveAction { cost: 1.0, offset: vec3i!(1, 0, 0) },
        MoveAction { cost: 1.0, offset: vec3i!(-1, 0, 0) },
        MoveAction { cost: 1.0, offset: vec3i!(0, 0, 1) },
        MoveAction { cost: 1.0, offset: vec3i!(0, 0, -1) },
    ]
}

pub fn moveset_2d_cardinal() -> Vec<MoveAction<Vector2i>> {
    vec![
        MoveAction { cost: 1.0, offset: vec2i!(1, 0) },
        MoveAction { cost: 1.0, offset: vec2i!(-1, 0) },
        MoveAction { cost: 1.0, offset: vec2i!(0, 1) },
        MoveAction { cost: 1.0, offset: vec2i!(0, -1) },
    ]
}