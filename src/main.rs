use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
struct Node {
    id: String,
    neighbors: Vec<Weak<Mutex<Node>>>, // weak for non-owning reference
}

impl Node {
    fn new(id: &str) -> Self {
        Node {
            id: String::from(id),
            neighbors: Vec::new(),
        }
    }

    fn get_neighbors_with_id(&self, target_id: &str) -> Vec<Arc<Mutex<Node>>> {
        self.neighbors.iter()
            .filter_map(|weak_neighbor| {
                match weak_neighbor.upgrade() {
                    Some(neighbor) => {
                        // Lock the Mutex and compare the id
                        if neighbor.lock().unwrap().id == target_id {
                            Some(neighbor)
                        } else {
                            None
                        }
                    },
                    None => {
                        println!("ERROR: fail to upgrade weak neighbor reference in node '{}' returned none, this should never happen, this means cleanup of a relation did not happen properly!", self.id);
                        None
                    }
                }
            })
            .collect()
    }
}

#[derive(Debug)]
struct Graph {
    nodes: BTreeMap<String, Arc<Mutex<Node>>>, // Arc for owning reference
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: BTreeMap::new(),
        }
    }

    fn add_node(&mut self, node: Node) -> Arc<Mutex<Node>> {
        let id = node.id.clone();
        let node_arc = Arc::new(Mutex::new(node));
        self.nodes.insert(id, node_arc.clone()); // clone the Arc, not the Node (increment reference counter)
        node_arc
    }

    fn add_edge(&mut self, from_node: &Arc<Mutex<Node>>, to_node: &Arc<Mutex<Node>>) {
        from_node.lock().unwrap().neighbors.push(Arc::downgrade(to_node));
        to_node.lock().unwrap().neighbors.push(Arc::downgrade(from_node));
    }

    fn get_by_id(&self, id: &str) -> Option<&Arc<Mutex<Node>>> {
        self.nodes.get(id)
    }
}

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
