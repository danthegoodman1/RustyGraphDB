use std::{sync::{Mutex, Arc}, collections::BTreeMap};

use crate::node::Node;

#[derive(Debug)]
pub struct Graph {
    nodes: BTreeMap<String, Arc<Mutex<Node>>>, // Arc for owning reference
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: BTreeMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Arc<Mutex<Node>> {
        let id = node.id.clone();
        let node_arc = Arc::new(Mutex::new(node));
        self.nodes.insert(id, node_arc.clone()); // clone the Arc, not the Node (increment reference counter)
        node_arc
    }

    pub fn add_edge(&mut self, from_node: &Arc<Mutex<Node>>, to_node: &Arc<Mutex<Node>>) {
        from_node.lock().unwrap().neighbors.push(Arc::downgrade(to_node));
        to_node.lock().unwrap().neighbors.push(Arc::downgrade(from_node));
    }

    pub fn get_by_id(&self, id: &str) -> Option<&Arc<Mutex<Node>>> {
        self.nodes.get(id)
    }
}
