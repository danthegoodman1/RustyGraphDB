pub mod graph;
pub mod node;

use std::time::Instant;

use graph::Graph;
use node::Node;

use crate::node::{RELATION_DIRECTION_FROM_ID, RELATION_DIRECTION_TO_ID};

// Traversed 10M in 40ms at 250.00M tps
// Traversed (with direction and kind) 10M in 340ms at 29.41M tps
// Traversed (direction only) 10M in 144ms at 69.44M tps

fn main() {
    let mut graph = Graph::new();

    let a_node = graph.add_node(Node::new("a"));
    let b_node = graph.add_node(Node::new("b"));
    let c_node = graph.add_node(Node::new("c"));

    // Make demo circular relation
    graph.add_relation(&a_node, &b_node, "friends");
    graph.add_relation(&b_node, &c_node, "friends");
    graph.add_relation(&c_node, &a_node, "friends");

    // Test getting a node in block so lock releases
    let node_a = graph
        .get_by_id("a")
        .expect("did not find node a!")
        .borrow();
    println!("Got node a id '{}'", node_a.id);

    // Test getting list of nodes
    let nodes = node_a.get_neighbors_with_id("b");
    // Build the node IDs
    let mut node_ids = Vec::new();
    for rc in nodes {
        node_ids.push(rc.borrow().id.clone())
    }
    dbg!(node_ids);

    // Get outgoing relations
    let node_b = b_node.borrow();
    let outgoing = node_b.get_relation(Option::Some(RELATION_DIRECTION_TO_ID), None);
    for i in outgoing {
        println!(
            "Got outgoing direction {} for id {}",
            i.direction,
            i.node()
                .upgrade()
                .expect("failed to upgrade!")
                .borrow()
                .id
        );
    }

    // Get incoming relations
    let incoming = node_b.get_relation(Option::Some(RELATION_DIRECTION_FROM_ID), None);
    for i in incoming {
        println!(
            "Got incoming direction {} for id {}",
            i.direction,
            i.node()
                .upgrade()
                .expect("failed to upgrade!")
                .borrow()
                .id
        );
    }

    // Test relation matching
    let a_friends = node_a.get_relation(None, Option::Some(String::from("friends")));
    if a_friends.len() != 2 {
        dbg!(a_friends);
        panic!("incorrect friends relation length")
    } else {
        println!("matched relation kind")
    }

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

    // Performance test conditional iteration 10M times, both matches
    let start = Instant::now();
    let mut current = a_node.clone();

    for _ in 0..10_000_000 {
        // scope because current is borroed
        let next_node = {
            let read = &current.borrow();
            let friends = read.get_relation(
                Option::Some(RELATION_DIRECTION_TO_ID),
                Option::Some(String::from("friends")),
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

    // Performance test partial conditional iteration 10M times, both matches
    let start = Instant::now();
    let mut current = a_node.clone();

    for _ in 0..10_000_000 {
        // scope because current is borroed
        let next_node = {
            let read = &current.borrow();
            let friends = read.get_relation(
                Option::Some(RELATION_DIRECTION_TO_ID),
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
}
