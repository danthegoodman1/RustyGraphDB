use std::{
    cell::RefCell, // Add this import
    fmt,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Node {
    pub id: [u8; 128], // Changed from String to fixed-size byte array
    pub neighbors: Vec<Relation>, // unchanged
}

pub const RELATION_DIRECTION_TO: u8 = 0;
pub const RELATION_DIRECTION_FROM: u8 = 1;

impl Node {
    pub fn new(id: [u8; 128]) -> Self { // Updated to accept byte array reference
        Node {
            id: id, // Copy the byte array
            neighbors: Vec::new(),
        }
    }

    pub fn get_neighbors_with_id(&self, target_id: &[u8; 128]) -> Vec<Rc<RefCell<Node>>> {
        self.neighbors.iter()
            .filter_map(|weak_neighbor| {
                match weak_neighbor.node().upgrade() {
                    Some(neighbor) => {
                        // Compare the id
                        if neighbor.borrow().id == *target_id {
                            Some(neighbor)
                        } else {
                            None
                        }
                    },
                    None => {
                        println!("ERROR: fail to upgrade weak neighbor reference in node '{}' returned none, this should never happen, this means cleanup of a relation did not happen properly!", String::from_utf8_lossy(&self.id));
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
        kind: Option<[u8; 128]>,
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
            RelationDirection::To(_) => RELATION_DIRECTION_TO,
            RelationDirection::From(_) => RELATION_DIRECTION_FROM,
        }
    }
}

#[derive(Debug)]
pub struct Relation {
    pub direction: RelationDirection,
    pub kind: [u8; 128],
}

impl Relation {
    pub fn new(direction: RelationDirection, kind: [u8; 128]) -> Self {
        Relation {
            direction,
            kind:  kind,
        }
    }

    /// Returns the node regardless of the direction
    pub fn node(&self) -> &Weak<RefCell<Node>> {
        // Update return type
        match &self.direction {
            RelationDirection::From(node) => node,
            RelationDirection::To(node) => node,
        }
    }
}
