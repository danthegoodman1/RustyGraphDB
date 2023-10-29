pub mod graph;
pub mod node;

use std::time::Instant;

use graph::Graph;
use node::Node;

use crate::node::{RELATION_DIRECTION_TO_ID, RELATION_DIRECTION_FROM_ID};

fn main() {
    let mut graph = Graph::new();

    let a_node = graph.add_node(Node::new("a"));
    let b_node = graph.add_node(Node::new("b"));
    let c_node = graph.add_node(Node::new("c"));

    // Make demo circular relation
    graph.add_relation(&a_node, &b_node, "friends");
    graph.add_relation(&b_node, &c_node, "friends");
    graph.add_relation(&c_node, &a_node, "family");

    // Test getting a node in block so lock releases
    let node_a = graph
        .get_by_id("a")
        .expect("did not find node a!")
        .read()
        .unwrap();
    println!("Got node a id '{}'", node_a.id);

    // Test getting list of nodes
    let nodes = node_a.get_neighbors_with_id("b");
    // Build the node IDs
    let mut node_ids = Vec::new();
    for arc in nodes {
        let node = arc.read().unwrap();
        node_ids.push(node.id.clone())
    }
    dbg!(node_ids);

    // Get outgoing relations
    let node_b = b_node.read().unwrap();
    let outgoing = node_b.get_with_relation(Option::Some(RELATION_DIRECTION_TO_ID), None);
    for i in outgoing {
        println!(
            "Got outgoing direction {} for id {}",
            i.direction,
            i.any_direction()
                .upgrade()
                .expect("failed to upgrade!")
                .read()
                .unwrap()
                .id
        );
    }

    // Get incoming relations
    let incoming = node_b.get_with_relation(Option::Some(RELATION_DIRECTION_FROM_ID), None);
    for i in incoming {
        println!(
            "Got incoming direction {} for id {}",
            i.direction,
            i.any_direction()
                .upgrade()
                .expect("failed to upgrade!")
                .read()
                .unwrap()
                .id
        );
    }

    // Test relation matching
    let a_friends = node_a.get_with_relation(None, Option::Some(String::from("friends")));
    if a_friends.len() != 1 {
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
            let relation = &current.read().expect("failed to read").neighbors[0];
            let direction = relation.any_direction();
            direction.upgrade().expect("failed to upgrade!")
        };
        current = next_node;
    }

    let end = start.elapsed().as_millis();
    println!("Traversed 10M in {}ms", end);
    // debug: Traversed 10M in 1149ms --- release: Traversed 10M in 334ms


    // let mut i = 0;
    // while start.elapsed().as_millis() < 1000 {
    //     // scope because current is borroed
    //     let next_node = {
    //         let relation = &current.read().expect("failed to read").neighbors[0];
    //         let direction = relation.any_direction();
    //         direction.upgrade().expect("failed to upgrade!")
    //     };
    //     current = next_node;
    //     i+=1;
    // }
    // println!("Traversed {} in 1 second", i);
    // release: Traversed 14584449 in 1 second
    // I think we are spending half the time incrementing
}
