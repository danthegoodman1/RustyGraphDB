pub mod node;
pub mod graph;

use node::Node;
use graph::Graph;



fn main() {
    let mut graph = Graph::new();

    let a_node = graph.add_node(Node::new("a"));
    let b_node = graph.add_node(Node::new("b"));
    let c_node = graph.add_node(Node::new("c"));

    graph.add_edge(&a_node, &b_node);
    graph.add_edge(&b_node, &c_node);

    // Test getting a node
    let node_a = graph.get_by_id("a").expect("did not find node a!").lock().unwrap();
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
}
