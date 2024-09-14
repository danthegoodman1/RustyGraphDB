pub mod graph;
pub mod node;

use std::time::Instant;

use graph::Graph;
use node::Node;

use crate::node::{RELATION_DIRECTION_FROM, RELATION_DIRECTION_TO};

// Traversed 10M in 40ms at 250.00M tps
// Traversed (direction only) 10M in 155ms at 64.52M tps
// Traversed (with direction and kind) 10M in 220ms at 45.45M tps

trait ToByteArray {
    fn to_byte_array(&self) -> [u8; 128];
}

impl ToByteArray for str {
    fn to_byte_array(&self) -> [u8; 128] {
        let mut array = [0u8; 128];
        let bytes = self.as_bytes();
        let len = bytes.len().min(128);
        array[..len].copy_from_slice(&bytes[..len]);
        array
    }
}


fn main() {
    let mut graph = Graph::new();

    // Add nodes with byte conversions
    let a_node = graph.add_node(Node::new("a".to_byte_array()));
    let b_node = graph.add_node(Node::new("b".to_byte_array()));
    let c_node = graph.add_node(Node::new("c".to_byte_array()));

    // Make demo circular relation
    graph.add_relation(&a_node, &b_node, "friends".to_byte_array());
    graph.add_relation(&b_node, &c_node, "friends".to_byte_array());
    graph.add_relation(&c_node, &a_node, "friends".to_byte_array());

    // Test getting a node in block so lock releases
    let node_a = graph
            .get_by_id(&"a".to_byte_array())
        .expect("did not find node a!")
        .borrow();
    println!("Got node a id '{:?}'", node_a.id);

    // Test getting list of nodes
    let nodes = node_a.get_neighbors_with_id(&"b".to_byte_array());

    // Get outgoing relations
    let node_b = b_node.borrow();
    let outgoing = node_b.get_relation(Option::Some(RELATION_DIRECTION_TO), None);
    for i in outgoing {
        println!(
            "Got outgoing direction {} for id {:?}",
            i.direction,
            i.node()
                .upgrade()
                .expect("failed to upgrade!")
                .borrow()
                .id
        );
    }

    // Get incoming relations
    let incoming = node_b.get_relation(Option::Some(RELATION_DIRECTION_FROM), None);
    for i in incoming {
        println!(
            "Got incoming direction {} for id {:?}",
            i.direction,
            i.node()
                .upgrade()
                .expect("failed to upgrade!")
                .borrow()
                .id
        );
    }

    // Test relation matching
    let a_friends = node_a.get_relation(None, Option::Some("friends".to_byte_array()));
    
    // Traverse 10M times
    let start = Instant::now();
    let mut current = a_node.clone();

    for _ in 0..10_000_000 {
        // scope because current is borroed
        let next_node = {
            let relation = &current.borrow().neighbors[0];
            let direction = relation.node();
            direction.upgrade().expect("failed to upgrade!")
        };
        current = next_node;
    }

    let end = start.elapsed().as_millis();
    println!("Traversed 10M in {}ms at {:.2}M tps", end, 10_000.0 / end as f64);

    // Performance test partial conditional iteration 10M times, both matches
    let start = Instant::now();
    let mut current = a_node.clone();

    for _ in 0..10_000_000 {
        // scope because current is borroed
        let next_node = {
            let read = &current.borrow();
            let friends = read.get_relation(
                Option::Some(RELATION_DIRECTION_TO),
                None,
            );
            friends[0]
                .node()
                .upgrade()
                .expect("failed to upgrade any direction")
        };
        current = next_node;
    }

    let end = start.elapsed().as_millis();
    println!("Traversed (direction only) 10M in {}ms at {:.2}M tps", end, 10_000.0 / end as f64);

    // Performance test conditional iteration 10M times, both matches
    let start = Instant::now();
    let mut current = a_node.clone();

    for _ in 0..10_000_000 {
        // scope because current is borroed
        let next_node = {
            let read = &current.borrow();
            let friends = read.get_relation(
                Option::Some(RELATION_DIRECTION_TO),
                Option::Some("friends".to_byte_array()),
            );
            friends[0]
                .node()
                .upgrade()
                .expect("failed to upgrade any direction")
        };
        current = next_node;
    }

    let end = start.elapsed().as_millis();
    println!("Traversed (with direction and kind) 10M in {}ms at {:.2}M tps", end, 10_000.0 / end as f64);
}
