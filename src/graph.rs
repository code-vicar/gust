use std::collections::HashMap;
use std::collections::hash_map::Keys;

use super::traits::NodeID;
use super::edge::*;

#[derive(Debug)]
pub struct Graph<T>
where
  T: NodeID {
    edges: EdgeMap<T>,
    adjacencies: HashMap<T::ID_TYPE, Vec<(EdgeKey, PathDirection)>>,
}

impl<T> Graph<T>
where
  T: NodeID {
  pub fn new() -> Graph<T> {
    Graph {
      edges: EdgeMap::new(),
      adjacencies: HashMap::new(),
    }
  }

  fn add_edge_base(&mut self, from: T::ID_TYPE, to: T::ID_TYPE, bidi: bool) -> EdgeKey {
    let key = self.edges.insert(Edge::new(
      from.clone(),
      to.clone(),
      bidi,
    ));
    match self.adjacencies.get_mut(&from) {
      Some(adjacencies) => {
        adjacencies.push((key, PathDirection::Forward));
      },
      None => {
        self.adjacencies.insert(from, vec![(key, PathDirection::Forward)]);
      }
    }
    if bidi {
      match self.adjacencies.get_mut(&to) {
        Some(edge_keys) => {
          edge_keys.push((key, PathDirection::Backward));
        },
        None => {
          self.adjacencies.insert(to, vec![(key, PathDirection::Backward)]);
        }
      }
    }
    key
  }

  pub fn add_edge(&mut self, from: T::ID_TYPE, to: T::ID_TYPE) -> EdgeKey {
    self.add_edge_base(from, to, false)
  }

  pub fn add_edge_bidi(&mut self, from: T::ID_TYPE, to: T::ID_TYPE) -> EdgeKey {
    self.add_edge_base(from, to, true)
  }

  pub fn length(&self) -> usize {
    self.adjacencies.keys().len()
  }

  pub fn vertices(&self) -> Keys<T::ID_TYPE, Vec<(EdgeKey, PathDirection)>> {
    self.adjacencies.keys()
  }

  pub fn get_adjacent(&self, id: &T::ID_TYPE) -> Vec<&<T as NodeID>::ID_TYPE> {
    match self.adjacencies.get(id) {
      Some(adjacencies) => {
        let mut paths = Vec::new();
        for (edge_key, dir) in adjacencies {
          let edge = self.edges.get(edge_key).unwrap();
          paths.push(&edge.get_path(dir).to);
        }
        paths
      }
      None => Vec::new()
    }
  }

  pub fn get_edge(&self, from: &T::ID_TYPE, to: &T::ID_TYPE) -> Option<&Edge<T>> {
    match self.adjacencies.get(from) {
      Some(adjacencies) => {
        for (edge_key, dir) in adjacencies {
          let edge = self.edges.get(edge_key).unwrap();
          let path = edge.get_path(dir);
          if &path.to == to {
            return Some(edge);
          }
        }
        None
      }
      None => None
    }
  }
}
