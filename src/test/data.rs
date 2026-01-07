use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Instant;
use crate::pathing::data::{BinaryHeapOpenSet, Node};
use crate::pathing::math::Vector2i;
use eyre::{Report, Result};
use log::info;
use rand::rngs::{SmallRng, StdRng};
use rand::{Rng, SeedableRng};
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
    assert!(lowest.is_some(), "Heap did not successfully pop off a value");
    assert_eq!(lowest.unwrap().f_cost(), 0.0, "Heap did not pop off the lowest value");
    let actual2 = hs.heap.cost_order();
    let intended2 = vec![1.0, 3.0, 2.0, 5.0, 4.0];
    assert_eq!(actual2, intended2, "Heap did not match the binary heap structure")
}

#[test]
fn against_std_1000() {
    against_std_range(1_000);
}

#[test]
fn against_std_10000() {
    against_std_range(10_000);
}

#[test]
fn against_std_100000() {
    against_std_range(100_000);
}

fn against_std_range(entries: usize) {
    let mut bh_std = BinaryHeap::new();
    let mut bh_custom = BinaryHeapOpenSet::new();
    let mut rng = SmallRng::seed_from_u64(1);
    let start_time = Instant::now();

    for _ in 0..entries {
        let node = Node {
            g_cost: rng.random_range(0..1000) as f64,
            h_cost: rng.random_range(0..1000) as f64,
            action: SpatialAction::new_root(Vector2i::zero()),
            parent: None,
            heap_idx: None
        };
        bh_std.push(Reverse(node));
        let res = bh_custom.insert_direct(node);
        assert!(res.is_ok(), "Failed to insert into BinaryHeapOpenSet: {}", res.unwrap_err())
    }

    let val_std = bh_std.into_vec();
    let val_custom = bh_custom.clone_data();

    assert_eq!(val_std.len(), val_custom.len(), "Heaps did not have a matching length");
    for i in 0..entries {
        assert_eq!(val_std[i].0, val_custom[i], "Values did not match: {:?} (std::BinaryHeap), {:?} (BinaryHeapOpenSet)",
                   val_std[i].0, val_custom[i]
        );
    }

    println!("std comparison took {} ms", start_time.elapsed().as_millis())
}

struct HeapScenario {
    heap: BinaryHeapOpenSet<Vector2i>
}

impl HeapScenario {
    fn new() -> Self {
        Self { heap: BinaryHeapOpenSet::new() }
    }
    fn dummy_node(&mut self, g_cost: f64, h_cost: f64) -> Result<()> {
        self.heap.insert_direct(Node {
            g_cost,
            h_cost,
            action: SpatialAction::new_root(Vector2i::zero()),
            parent: None,
            heap_idx: None,
        })
    }
}