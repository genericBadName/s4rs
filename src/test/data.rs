use crate::pathing::data::{BinaryHeapOpenSet, Node};
use crate::pathing::math::Vector2i;
use eyre::{Report, Result};
use crate::pathing::action::SpatialAction;

#[test]
fn heap_ordered() {
    let mut hs = HeapScenario::new();
    let mut results = vec![
        hs.dummy_node(0.0, 0.0),
        hs.dummy_node(1.0, 0.0),
        hs.dummy_node(2.0, 0.0)
    ];
    results.retain(|r| r.is_err());

    assert!(results.is_empty(), "A heap operation failed: {}", results[0].as_ref().unwrap_err());
    let actual = hs.heap.cost_order();
    let intended = vec![0.0, 1.0, 2.0];
    assert_eq!(actual, intended, "Heap did not match the binary heap structure")
}

#[test]
fn heap_unordered() {
    let mut hs = HeapScenario::new();
    let mut results = vec![
        hs.dummy_node(1.0, 0.0),
        hs.dummy_node(3.0, 0.0),
        hs.dummy_node(2.0, 0.0),
        hs.dummy_node(5.0, 0.0),
        hs.dummy_node(0.0, 0.0),
        hs.dummy_node(4.0, 0.0)
    ];
    results.retain(|r| r.is_err());
    assert!(results.is_empty(), "A heap operation failed: {}", results[0].as_ref().unwrap_err());
    let actual = hs.heap.cost_order();
    let intended = vec![0.0, 1.0, 2.0, 5.0, 3.0, 4.0];
    assert_eq!(actual, intended, "Heap did not match the binary heap structure");

    let lowest = hs.heap.pop();
    assert!(lowest.is_ok(), "Heap did not successfully pop off a value");
    assert_eq!(lowest.unwrap().f_cost(), 0.0, "Heap did not pop off the lowest value");
    let actual2 = hs.heap.cost_order();
    let intended2 = vec![1.0, 3.0, 2.0, 5.0, 4.0];
    assert_eq!(actual2, intended2, "Heap did not match the binary heap structure")
}

struct HeapScenario {
    heap: BinaryHeapOpenSet<Vector2i>
}

impl HeapScenario {
    fn new() -> Self {
        Self { heap: BinaryHeapOpenSet::new() }
    }
    fn dummy_node(&mut self, g_cost: f64, h_cost: f64) -> Result<()> {
        self.heap.insert(Node {
            g_cost,
            h_cost,
            action: SpatialAction::new_root(Vector2i::zero()),
            parent: None,
            heap_idx: None,
        })
    }
}