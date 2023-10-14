use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
};

use crate::node::{Node, Relation, RelationDirection};

#[derive(Debug)]
pub struct Graph {
    nodes: BTreeMap<String, Arc<RwLock<Node>>>, // Arc for owning reference
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: BTreeMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Arc<RwLock<Node>> {
        let id = node.id.clone();
        let node_arc = Arc::new(RwLock::new(node));
        self.nodes.insert(id, node_arc.clone()); // clone the Arc, not the Node (increment reference counter)
        node_arc
    }

    pub fn add_relation(
        &mut self,
        from_node: &Arc<RwLock<Node>>,
        to_node: &Arc<RwLock<Node>>,
        relation: &str,
    ) {
        from_node.write().unwrap().neighbors.push(Relation::new(
            RelationDirection::To(Arc::downgrade(to_node)),
            relation,
        ));
        to_node.write().unwrap().neighbors.push(Relation::new(
            RelationDirection::From(Arc::downgrade(from_node)),
            relation,
        ));
    }

    pub fn get_by_id(&self, id: &str) -> Option<&Arc<RwLock<Node>>> {
        self.nodes.get(id)
    }
}
