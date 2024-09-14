use std::{
    fmt, 
    rc::{Rc, Weak},
    cell::RefCell, // Add this import
};

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub neighbors: Vec<Relation>, // weak for non-owning reference
}

pub const RELATION_DIRECTION_TO_ID: u8 = 0;
pub const RELATION_DIRECTION_FROM_ID: u8 = 1;

impl Node {
    pub fn new(id: &str) -> Self {
        Node {
            id: String::from(id),
            neighbors: Vec::new(),
        }
    }

    pub fn get_neighbors_with_id(&self, target_id: &str) -> Vec<Rc<RefCell<Node>>> {
        self.neighbors.iter()
            .filter_map(|weak_neighbor| {
                match weak_neighbor.node().upgrade() {
                    Some(neighbor) => {
                        // Lock the Mutex and compare the id
                        if neighbor.borrow().id == target_id {
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

    /// Gets relations matching one or both conditions
    pub fn get_relation(
        &self,
        relation_direction: Option<u8>,
        kind: Option<String>,
    ) -> Vec<&Relation> {
        self.neighbors
            .iter()
            .filter(|weak_neighbor| {
                // Default true for AND operation (since ignored if not included)
                let mut rel_valid = true;
                let mut kind_valid = true;

                // Check for relation direction match
                if let Some(rel_dir) = &relation_direction {
                    rel_valid = rel_dir == &weak_neighbor.direction.id();
                }

                // Check for kind match
                if let Some(k) = &kind {
                    kind_valid = k == &weak_neighbor.kind;
                }

                rel_valid && kind_valid
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum RelationDirection {
    From(Weak<RefCell<Node>>), // Change Weak<Node> to Weak<RefCell<Node>>
    To(Weak<RefCell<Node>>),   // Change Weak<Node> to Weak<RefCell<Node>>
}

impl fmt::Display for RelationDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl RelationDirection {
    fn id(&self) -> u8 {
        match self {
            RelationDirection::To(_) => RELATION_DIRECTION_TO_ID,
            RelationDirection::From(_) => RELATION_DIRECTION_FROM_ID,
        }
    }
}

#[derive(Debug)]
pub struct Relation {
    pub direction: RelationDirection,
    pub kind: String,
}

impl Relation {
    pub fn new(direction: RelationDirection, kind: &str) -> Self {
        Relation {
            direction,
            kind: String::from(kind),
        }
    }

    /// Returns the node regardless of the direction
    pub fn node(&self) -> &Weak<RefCell<Node>> { // Update return type
        match &self.direction {
            RelationDirection::From(node) => node,
            RelationDirection::To(node) => node,
        }
    }
}
