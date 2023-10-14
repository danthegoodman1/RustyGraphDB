pub mod graph;
pub mod node;

use graph::Graph;
use node::Node;

fn main() {
    let mut graph = Graph::new();

    let a_node = graph.add_node(Node::new("a"));
    let b_node = graph.add_node(Node::new("b"));
    let c_node = graph.add_node(Node::new("c"));

    graph.add_relation(&a_node, &b_node, "friends");
    graph.add_relation(&b_node, &c_node, "friends");

    // Test getting a node
    let node_a = graph
        .get_by_id("a")
        .expect("did not find node a!")
        .lock()
        .unwrap();
    println!("Got node a id '{}'", node_a.id);

    // Test getting list of nodes
    let nodes = node_a.get_neighbors_with_id("b");
    // Build the node IDs
    let mut node_ids = Vec::new();
    for arc in nodes {
        let node = arc.lock().unwrap();
        node_ids.push(node.id.clone())
    }
    dbg!(node_ids);

    // Get outgoing relations
    let node_b = b_node.lock().unwrap();
    let outgoing = node_b.get_outgoing_relations();
    dbg!(outgoing);

    // Get incoming relations
    let incoming = node_b.get_incoming_relations();
    dbg!(incoming);
}
