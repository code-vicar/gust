use std::collections::HashMap;

use super::traits::HasID;
use super::edge::Edge;

#[derive(Debug)]
pub struct Graph<T, X>
where T: HasID {
  vertices: HashMap<T::ID_TYPE, T>,
  adjacencies: HashMap<T::ID_TYPE, Vec<Edge<T, X>>>,
}

impl<T, X> Graph<T, X>
where T: HasID {
  pub fn new(graph_options: GraphBuilder<T>) -> Graph<T, X> {
    let vertices = graph_options.vertices;
    let adjacencies = HashMap::new();
    Graph { vertices, adjacencies }
  }

  pub fn add_vertex(&mut self, vertex: T) {
    self.vertices.insert(vertex.get_id().to_owned(), vertex);
  }

  fn can_connect(&self, left: &T::ID_TYPE, right: &T::ID_TYPE) -> bool {
    self.vertices.contains_key(&left) && self.vertices.contains_key(&right)
  }

  pub fn add_edge_with_data(&mut self, left: T::ID_TYPE, right: T::ID_TYPE, edge_data: Option<X>) -> bool {
    if !self.can_connect(&left, &right) {
      return false
    }
    let edge = Edge {
      from: left.clone(),
      to: right,
      meta: edge_data
    };
    match self.adjacencies.get_mut(&left) {
      Some(a_adjacent) => {
        a_adjacent.push(edge);
      }
      None => {
        self.adjacencies.insert(left, vec![edge]);
      }
    }
    return true
  }

  // Adds edge between left and right vertex, one direction.
  pub fn add_edge(&mut self, left: T::ID_TYPE, right: T::ID_TYPE) -> bool {
    self.add_edge_with_data(left, right, None)
  }

  pub fn length(&self) -> usize {
    self.vertices.len()
  }

  pub fn vertices(&self) -> &HashMap<T::ID_TYPE, T> {
    let ref v = self.vertices;
    v
  }
}

impl<T, X> Graph<T, X> where
  T: HasID + Clone,
{
  pub fn new_clone(graph_options: &GraphBuilder<T>) -> Graph<T, X> {
    let graph_options_clone = graph_options.clone();
    let vertices = graph_options_clone.vertices;
    let adjacencies = HashMap::new();
    Graph { vertices, adjacencies }
  }
}

/*
*  Graph Builder
*/
pub struct GraphBuilder<T: HasID> {
  vertices: HashMap<T::ID_TYPE, T>
}

impl<T> Clone for GraphBuilder<T> where
  T: HasID + Clone,
{
  fn clone(&self) -> Self {
    GraphBuilder {
      vertices: self.vertices.clone()
    }
  }
}

impl<T: HasID> GraphBuilder<T> {
  pub fn new() -> GraphBuilder<T> {
    GraphBuilder {
      vertices: HashMap::new()
    }
  }

  pub fn with_vertex(mut self, v: T) -> GraphBuilder<T> {
    self.vertices.insert(v.get_id().to_owned(), v);
    self
  }

  pub fn with_vertices(mut self, vertices: Vec<T>) -> GraphBuilder<T> {
    for v in vertices {
      self.vertices.insert(v.get_id().to_owned(), v);
    }
    self
  }

  pub fn build<X>(self) -> Graph<T, X> {
    Graph::new(self)
  }
}

impl <T: HasID + Clone> GraphBuilder<T> {
  pub fn with_vertex_clone(mut self, v: &T) -> GraphBuilder<T> {
    let v_clone = v.clone();
    self.vertices.insert(v_clone.get_id().to_owned(), v_clone);
    self
  }

  pub fn with_vertices_clone(mut self, vertices: &Vec<T>) -> GraphBuilder<T> {
    let cloned = vertices.clone();
    for v in cloned {
      self.vertices.insert(v.get_id().to_owned(), v);
    }
    self
  }

  pub fn build_clone<X>(&self) -> Graph<T, X> {
    Graph::new_clone(self)
  }
}
