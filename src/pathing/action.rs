use crate::pathing::math::{Vector2i, Vector3i};
use crate::{vec2i, vec3i};
use std::ops::Add;
use jni::JNIEnv;
use jni::objects::JObject;
use crate::binding::jni::JNICompatible;
use crate::pathing::algorithm::GraphPosition;

pub type Moveset<P>  = Vec<MoveAction<P>>;

/// A move action that can be taken by the pathfinding entity. These are the "lines" that
/// connect nodes on the graph.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct MoveAction<P> where P: GraphPosition {
    /// Initial cost to execute this move action.
    pub cost: u64,
    /// Offset from the current position to check for this action (the neighbor position).
    pub offset: P
}

impl <P> MoveAction<P> where P: GraphPosition {
    pub const fn new(cost: u64, offset: P) -> Self {
        Self { cost, offset }
    }
}

/// Represents an action taken to move to a point in space.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct SpatialAction<P> where P: GraphPosition {
    /// The position that the action ended up at.
    pub pos: P,
    /// The type of action itself. If `Option::None`, this is the root action (where the pathfinder
    /// started from).
    pub move_action: Option<MoveAction<P>>
}

impl <P> SpatialAction<P> where P: GraphPosition {
    pub const fn new(pos: P, move_action: MoveAction<P>) -> Self {
        Self { pos, move_action: Some(move_action) }
    }

    pub const fn new_root(pos: P) -> Self {
        Self { pos, move_action: None }
    }
}

impl <'local, P> JNICompatible<'local> for SpatialAction<P> where P: GraphPosition {
    const CLASS: &'static str = "com/genericbadname/s4mc/pathing/SpatialAction";

    fn to_jni(&self, env: &mut JNIEnv<'local>) -> eyre::Result<JObject<'local>> {
        todo!()
    }

    fn from_jni(env: &mut JNIEnv<'local>, object: JObject<'local>) -> eyre::Result<Self>
    where
        Self: Sized
    {
        todo!()
    }
}

pub fn default_moveset() -> Moveset<Vector3i> {
    vec![
        MoveAction::new(1, vec3i!(1, 0, 0)),
        MoveAction::new(1, vec3i!(-1, 0, 0)),
        MoveAction::new(1, vec3i!(0, 0, 1)),
        MoveAction::new(1, vec3i!(0, 0, -1))
    ]
}

pub fn moveset_2d_cardinal() -> Moveset<Vector2i> {
    vec![
        MoveAction::new(1, vec2i!(1, 0)),
        MoveAction::new(1, vec2i!(-1, 0)),
        MoveAction::new(1, vec2i!(0, 1)),
        MoveAction::new(1, vec2i!(0, -1))
    ]
}

pub enum Moveset2D {
    Left,
    Right,
    Up,
    Down
}

impl Moveset2D {
    pub const fn of(&self) -> MoveAction<Vector2i> {
        match self {
            Moveset2D::Left => MoveAction::new(1, vec2i!(-1, 0)),
            Moveset2D::Right => MoveAction::new(1, vec2i!(1, 0)),
            Moveset2D::Up => MoveAction::new(1, vec2i!(0, -1)),
            Moveset2D::Down => MoveAction::new(1, vec2i!(0, 1))
        }
    }
}

impl Into<MoveAction<Vector2i>> for Moveset2D {
    fn into(self) -> MoveAction<Vector2i> {
        self.of()
    }
}