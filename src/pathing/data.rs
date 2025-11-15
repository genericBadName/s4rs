use crate::binding::jni::JNICompatible;
use eyre::Result;
use jni::objects::{JObject, JValueGen};
use jni::JNIEnv;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::ops::Add;
use std::rc::Rc;
use crate::pathing::algorithm::GraphPosition;

/// A node within the A* graph.
#[derive(Debug, Clone)]
pub struct Node<P> where P: GraphPosition
{
    /// Initial cost to traverse to this node.
    pub g_cost: f64,
    /// Heuristic cost to traverse to this node.
    pub h_cost: f64,
    /// The node's position in space.
    pub pos: P,
    /// Parent of this node. If `Option::None`, this is considered the root node.
    pub parent: Option<Rc<Node<P>>>
}
impl <P> Node<P> where P: GraphPosition
{
    pub fn start_node(start: P, end: &P) -> Node<P> {
        Node {
            g_cost: 0.0,
            h_cost: start.heuristic_from_self(end),
            parent: None,
            pos: start
        }
    }
    
    pub fn f_cost(&self) -> f64 {
        self.g_cost + self.h_cost
    }
}

impl <'a, P> PartialEq<Self> for Node<P> where P: GraphPosition
{
    fn eq(&self, other: &Self) -> bool {
        self.f_cost().eq(&other.f_cost())
    }
}

impl <'a, P> PartialOrd for Node<P> where P: GraphPosition
{
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

impl <P> Eq for Node<P> where P: GraphPosition {}
impl <P> Ord for Node<P> where P: GraphPosition
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_cost().total_cmp(&other.f_cost())
    }
}

/// A node on the path to a given destination. Does not contain unnecessary cost data like with
/// `Node`, only movement-related information.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PathNode<P> where P: GraphPosition
{
    pub pos: P
}

impl <P> From<Node<P>> for PathNode<P> where P: GraphPosition
{
    fn from(value: Node<P>) -> Self {
        PathNode {
            pos: value.pos
        }
    }
}

impl <P> From<&Node<P>> for PathNode<P> where P: GraphPosition
{
    fn from(value: &Node<P>) -> Self {
        PathNode {
            pos: value.pos
        }
    }
}

impl <'local, P> JNICompatible<'local> for PathNode<P> where P: GraphPosition + JNICompatible<'local> {
    const CLASS: &'static str = "com/genericbadname/s4mc/pathing/PathNode";

    fn to_jni(&self, env: &mut JNIEnv<'local>) -> Result<JObject<'local>> {
        let pathnode_class = env.find_class(Self::CLASS)?;
        let pos = self.pos.to_jni(env)?;
        Ok(env.new_object(pathnode_class, "(Lcom/genericbadname/s4mc/math/Vector3i;)V", &[
            JValueGen::Object(&pos),
        ])?)
    }

    fn from_jni(env: &mut JNIEnv<'local>, object: JObject<'local>) -> Result<Self>
    where
        Self: Sized
    {
        let pos = env.call_method(
            &object, "pos", "()Lgenericbadname/s4mc/math/Vector3i;", &[]
        )?.l()?;

        Ok(Self {
            pos: P::from_jni(env, pos)?
        })
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