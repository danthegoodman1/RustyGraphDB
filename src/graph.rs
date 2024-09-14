use std::{
    collections::BTreeMap,
    rc::{Rc, Weak},
    cell::RefCell, // Add this import
};

use crate::node::{Node, Relation, RelationDirection};

#[derive(Debug)]
pub struct Graph {
    /// Note: this is not thread-safe. Either use a Rwlock on top, or SkipMap (https://tikv.github.io/doc/crossbeam_skiplist/index.html)
    nodes: BTreeMap<String, Rc<RefCell<Node>>>, // Change Rc<Node> to Rc<RefCell<Node>>
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: BTreeMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Rc<RefCell<Node>> { // Update return type
        let id = node.id.clone();
        let node_rc = Rc::new(RefCell::new(node)); // Wrap Node in RefCell
        self.nodes.insert(id, Rc::clone(&node_rc));
        node_rc
    }

    pub fn add_relation(
        &mut self,
        from_node: &Rc<RefCell<Node>>,
        to_node: &Rc<RefCell<Node>>,
        relation: &str,
    ) {
        from_node.borrow_mut().neighbors.push(Relation::new(
            RelationDirection::To(Rc::downgrade(to_node)),
            relation,
        ));
        to_node.borrow_mut().neighbors.push(Relation::new(
            RelationDirection::From(Rc::downgrade(from_node)),
            relation,
        ));
    }

    pub fn get_by_id(&self, id: &str) -> Option<&Rc<RefCell<Node>>> {
        self.nodes.get(id)
    }
}
