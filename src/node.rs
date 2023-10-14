use std::{sync::{Arc, Mutex, Weak}, fmt};

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub neighbors: Vec<Relation>, // weak for non-owning reference
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
                match weak_neighbor.any_direction().upgrade() {
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

    pub fn get_outgoing_relations(&self) -> Vec<&Relation> {
        self.neighbors
            .iter()
            .filter(|weak_neightbor| matches!(weak_neightbor.direction, RelationDirection::To(_)))
            .collect()
    }
    pub fn get_incoming_relations(&self) -> Vec<&Relation> {
        self.neighbors
            .iter()
            .filter(|weak_neightbor| matches!(weak_neightbor.direction, RelationDirection::From(_)))
            .collect()
    }
}

#[derive(Debug)]
pub enum RelationDirection {
    From(Weak<Mutex<Node>>),
    To(Weak<Mutex<Node>>),
}

// Custom display to make to_string() return the enum type name
impl fmt::Display for RelationDirection {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
  }
}

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

    /// Returns the direction regardless of type
    pub fn any_direction(&self) -> &Weak<Mutex<Node>> {
        match &self.direction {
            RelationDirection::From(node) => node,
            RelationDirection::To(node) => node,
        }
    }
}

impl fmt::Debug for Relation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let node_id = match self.direction {
          RelationDirection::From(ref weak) | RelationDirection::To(ref weak) => {
              weak.upgrade().map_or_else(
                  || String::from("(dangling reference)"),
                  |arc_node| arc_node.lock().unwrap().id.clone()
              )
          },
      };

      f.debug_struct("Relation")
          .field("direction", &format_args!("{}({})", self.direction.to_string(), node_id))
          .field("kind", &self.kind)
          .finish()
  }
}
