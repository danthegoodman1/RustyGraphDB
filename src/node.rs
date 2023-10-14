use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub neighbors: Vec<Weak<Mutex<Node>>>, // weak for non-owning reference
}

impl Node {
    pub fn new(id: &str) -> Self {
        Node {
            id: String::from(id),
            neighbors: Vec::new(),
        }
    }

    pub fn get_neighbors_with_id(&self, target_id: &str) -> Vec<Arc<Mutex<Node>>> {
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
