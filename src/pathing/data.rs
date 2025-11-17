use crate::binding::jni::JNICompatible;
use eyre::{eyre, OptionExt, Result};
use jni::objects::{JObject, JValueGen};
use jni::JNIEnv;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::pathing::action::SpatialAction;
use crate::pathing::algorithm::GraphPosition;
use crate::pathing::math::Vector2i;

/// A node within the A* graph.
#[derive(Debug, Copy, Clone)]
pub struct Node<P> where P: GraphPosition
{
    /// Initial cost to traverse to this node.
    pub g_cost: f64,
    /// Heuristic cost to traverse to this node.
    pub h_cost: f64,
    /// The node's position in space and how the pathfinder moved to it.
    pub action: SpatialAction<P>,
    /// Parent of this node. If `Option::None`, this is considered the root node.
    pub parent: Option<usize>,
    /// Open-set heap index. If `Option::None`, this `Node` is Closed.
    pub heap_idx: Option<usize>
}
impl <P> Node<P> where P: GraphPosition
{
    /// Constructs the root `Node`, which is has a root `SpatialAction` and a cost of 0.
    pub fn start_node(start: P, end: &P) -> Self<> {
        Self {
            g_cost: 0.0,
            h_cost: start.distance_to(end),
            parent: None,
            action: SpatialAction::new_root(start),
            heap_idx: None
        }
    }

    /// Returns the f-cost, which is simply the g-cost plus the h-cost.
    pub fn f_cost(&self) -> f64 {
        self.g_cost + self.h_cost
    }

    /// Returns whether this `Node` is Open, meaning it has a valid heap index.
    pub fn is_open(&self) -> bool {
        self.heap_idx.is_some()
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
impl <P> Ord for Node<P> where P: GraphPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_cost().total_cmp(&other.f_cost())
    }
}

/// A node on the path to a given destination. Does not contain unnecessary cost data like with
/// `Node`, only movement-related information.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PathNode<P> where P: GraphPosition
{
    pub action: SpatialAction<P>
}

impl <P> PathNode<P> where P: GraphPosition {
    pub fn new(action: SpatialAction<P>) -> Self {
        Self { action }
    }
}

impl <'local, P> JNICompatible<'local> for PathNode<P> where P: GraphPosition + JNICompatible<'local> {
    const CLASS: &'static str = "com/genericbadname/s4mc/pathing/PathNode";

    fn to_jni(&self, env: &mut JNIEnv<'local>) -> Result<JObject<'local>> {
        let pathnode_class = env.find_class(Self::CLASS)?;
        let pos = self.action.to_jni(env)?;
        Ok(env.new_object(pathnode_class, "(Lcom/genericbadname/s4mc/math/Vector3i;)V", &[
            JValueGen::Object(&pos),
        ])?)
    }

    fn from_jni(env: &mut JNIEnv<'local>, object: JObject<'local>) -> Result<Self>
    where
        Self: Sized
    {
        // TODO: update this!!
        let pos = env.call_method(
            &object, "pos", "()Lgenericbadname/s4mc/math/Vector3i;", &[]
        )?.l()?;

        Ok(Self {
            action: SpatialAction::from_jni(env, pos)?
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

/// An Open Set backed by a Binary Heap. Differs from `std::collections::BinaryHeap` in that it
/// implements decrease-key operations necessary for Dijkstra's algorithm to not have terrible
/// space complexity. This is an "implicit heap," that is, parent-child relationships are not
/// stored through pointers, only indices into a `Vec`.
///
/// This implementation handles events in which the keys on a `Node` are somehow mutated when they
/// shouldn't, or are mismatched from their real values. However, `Nodes` that modify themselves
/// through interior mutability patterns to change their cost/key are considered undefined
/// behavior.
pub struct BinaryHeapOpenSet<P> where P: GraphPosition {
    /// Underlying data structure of the set.
    data: Vec<Node<P>>
}

/// Simple struct for comparing keys & values in an Open Set.
#[derive(Debug, Copy, Clone)]
pub(crate) struct SetEntry {
    /// The index associated with the `Node`.
    pub(crate) idx: usize,
    /// The cost (priority) of the `Node`.
    pub(crate) cost: f64
}

impl SetEntry {
    pub(crate) fn new(idx: usize, cost: f64) -> Self {
        Self { idx, cost }
    }
}

impl <P> From<Node<P>> for SetEntry where P: GraphPosition {
    fn from(value: Node<P>) -> Self {
        Self::from(&value)
    }
}

impl <P> From<&Node<P>> for SetEntry where P: GraphPosition {
    fn from(value: &Node<P>) -> Self {
        Self {
            idx: value.heap_idx.unwrap(),
            cost: value.f_cost()
        }
    }
}

impl Display for SetEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(idx: {}, cost: {})", self.idx, self.cost)
    }
}

impl <P> BinaryHeapOpenSet<P> where P: GraphPosition {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(1024)
        }
    }

    /// Gets the parent index of the `idx` (a child)
    fn parent_of(idx: usize) -> usize {
        (idx.saturating_sub(1)) >> 1
    }

    /// Gets the left child of `idx` (a parent)
    fn child_of(idx: usize) -> usize {
        (idx << 1) + 1
    }

    /// Returns a `SetEntry` for the specified index, or an `Err` if none exists.
    fn entry_for(&self, idx: usize) -> Result<SetEntry> {
        let se = SetEntry::from(self.data.get(idx)
            .ok_or_else(|| eyre!("Couldn't get a node (index {})", idx))?);
        Ok(se)
    }

    /// Insert a new `Node` into the Open Set. Returns an `Err` if something wrong occurred while
    /// sifting `node` into the set.
    pub fn insert(&mut self, mut node: Node<P>) -> Result<()> {
        node.heap_idx = Some(self.data.len());
        self.data.push(node);
        self.sift_up(&node)
    }

    /// Sifts the set from `node` until it is leveled with the rest of the heap.
    /// Runs a decrease-key operation until fully leveled.
    pub fn sift_up(&mut self, to_update: &Node<P>) -> Result<()> {
        // do not operate on Closed nodes!!
        if to_update.heap_idx.is_none() { return Err(eyre!("Tried to sift a Closed node")) }
        if to_update.heap_idx.unwrap() == 0 { return Ok(()) }

        let mut updating = SetEntry::from(to_update);
        let mut parent = self.entry_for(Self::parent_of(updating.idx))?;

        // sift until the node reaches the correct position
        while updating.idx > 0 && parent.cost > updating.cost {
            // swap around
            self.swap_idx(updating.idx, parent.idx)?;
            // and then update our working values
            updating.idx = parent.idx;
            parent.idx = Self::parent_of(updating.idx);
            parent.cost = self.entry_for(parent.idx)?.cost
        }

        Ok(())
    }

    fn swap_idx(&mut self, a: usize, b: usize) -> Result<()> {
        self.data.swap(a, b);
        self.data.get_mut(a)
            .ok_or_else(|| eyre!("Couldn't get node (index {}) to update its heap index", a))?
            .heap_idx = Some(a);
        self.data.get_mut(b)
            .ok_or_else(|| eyre!("Couldn't get node (index {}) to update its heap index", b))?
            .heap_idx = Some(b);
        Ok(())
    }

    /// Returns whether the Open Set is empty or not.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Removes the lowest cost node from the set. Returns an `Ok(Node<P>)` with the `Node` if it
    /// was removed and the set was updated successfully.
    pub fn pop(&mut self) -> Result<Node<P>> {
        if self.is_empty() { return Err(eyre!("Empty set")) }
        let mut lowest = self.data.swap_remove(0);
        lowest.heap_idx = None; // node taken out: update its index!
        self.data.get_mut(0)
            .ok_or_eyre("Couldn't get the next lowest node (index 0)")?.heap_idx = Some(0);

        // if there's no more nodes, no need to sift any further
        if self.data.len() < 2 { return Ok(lowest) }

        // otherwise, keep trying to sift down
        self.sift_down().map(|_| lowest)
    }

    /// Sifts the heap from the top down, readjusting from the root `Node` until the heap
    /// is leveled out.
    fn sift_down(&mut self) -> Result<()> {
        let size = self.data.len();
        // parent, originally the topmost node in the tree
        let mut parent = self.entry_for(0)?;
        // the left child by default
        let mut child = SetEntry::new(1, 0.0);

        while child.idx <= size {
            child.cost = self.entry_for(child.idx)?.cost;
            // get which child (left or right) is better to sift them up
            if child.idx < size {
                let rc_idx = child.idx + 1;
                let rc_cost = self.entry_for(rc_idx)?.cost;
                if child.cost > rc_cost {
                    child.idx += 1;
                    child.cost = rc_cost;
                }
            }
            // don't swap if the heap is level now
            if parent.cost <= child.cost { break }
            // otherwise, update indices
            self.swap_idx(parent.idx, child.idx)?;
            parent.idx = child.idx;
            child.idx = Self::child_of(parent.idx);
        }

        Ok(())
    }

    /// Clears the set, completely emptying its contents.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Gets a reference to a `Node` at the specified index. Returns `Option::None` if that index
    /// has no associated value.
    pub fn get(&self, idx: usize) -> Option<&Node<P>> {
        self.data.get(idx)
    }

    /// Returns the list of costs within the heap.
    pub fn cost_order(&self) -> Vec<f64> {
        self.data.iter().map(|n| n.f_cost()).collect()
    }
}