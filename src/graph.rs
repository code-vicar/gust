use std::collections::HashMap;
use std::collections::hash_map::Keys;

use super::traits::HasID;
use super::edge::*;

#[derive(Debug)]
pub struct Graph<T>
where
  T: HasID {
    edges: EdgeMap<T>,
    adjacencies: HashMap<T::ID_TYPE, Vec<EdgeKey>>,
}

impl<T> Graph<T>
where
  T: HasID {
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
      Some(edge_keys) => {
        edge_keys.push(key);
      },
      None => {
        self.adjacencies.insert(from, vec![key]);
      }
    }
    if bidi {
      match self.adjacencies.get_mut(&to) {
        Some(edge_keys) => {
          edge_keys.push(key);
        },
        None => {
          self.adjacencies.insert(to, vec![key]);
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

  pub fn vertices(&self) -> Keys<T::ID_TYPE, Vec<EdgeKey>> {
    self.adjacencies.keys()
  }

  pub fn get_adjacent(&self, id: &T::ID_TYPE) -> Vec<&Edge<T>> {
    match self.adjacencies.get(id) {
      Some(edge_keys) => {
        let mut edges = Vec::new();
        for key in edge_keys {
          edges.push(self.edges.get(key).unwrap());
        }
        edges
      }
      None => Vec::new()
    }
  }

  pub fn has_edge(&self, from: &T::ID_TYPE, to: &T::ID_TYPE) -> bool {
    let edges = self.get_adjacent(from);
    edges.iter().any(|edge| edge.leads_to(to))
  }
}
