use super::traits::HasID;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Edge<T: HasID> {
  from: T::ID_TYPE,
  to: T::ID_TYPE,
  bidi: bool
}

impl<T: HasID> Edge<T> {
  pub fn new(from: T::ID_TYPE, to: T::ID_TYPE, bidi: bool) -> Edge<T> {
    Edge {
      from,
      to,
      bidi,
    }
  }

  pub fn leads_to(&self, vertex_id: &T::ID_TYPE) -> bool {
    if !self.bidi {
      return &self.to == vertex_id
    }
    return &self.to == vertex_id || &self.from == vertex_id
  }

  pub fn leads_from(&self, vertex_id: &T::ID_TYPE) -> bool {
    if !self.bidi {
      return &self.from == vertex_id
    }
    return &self.from == vertex_id || &self.to == vertex_id
  }

  pub fn leads_from_to(&self, from: &T::ID_TYPE, to: &T::ID_TYPE) -> bool {
    self.leads_from(from) && self.leads_to(to)
  }

  pub fn set_connection(&mut self, from: T::ID_TYPE, to: T::ID_TYPE) {
    self.from = from;
    self.to = to;
  }

  pub fn set_connection_bi(&mut self, from: T::ID_TYPE, to: T::ID_TYPE) {
    self.from = from;
    self.to = to;
    self.bidi = true;
  }
}

impl<T: HasID> Clone for Edge<T> {
  fn clone(&self) -> Self {
    Edge {
      from: self.from.clone(),
      to: self.to.clone(),
      bidi: self.bidi
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeKey {
  idx: usize,
  generation: usize,
}

#[derive(Debug)]
pub struct EdgeMap<T: HasID> {
  cursor: usize,
  free_pool: Vec<EdgeKey>,
  keys: HashMap<usize, EdgeKey>,
  internal_map: HashMap<usize, Edge<T>>
}

impl<T: HasID> Clone for EdgeMap<T> {
  fn clone(&self) -> Self {
    EdgeMap {
      cursor: self.cursor,
      free_pool: self.free_pool.clone(),
      keys: self.keys.clone(),
      internal_map: self.internal_map.clone()
    }
  }
}

impl<T: HasID> EdgeMap<T> {
  pub fn new() -> EdgeMap<T> {
    EdgeMap {
      cursor: 0,
      free_pool: Vec::new(),
      keys: HashMap::new(),
      internal_map: HashMap::new(),
    }
  }

  fn get_next_key(&mut self) -> EdgeKey {
    let next_id = self.free_pool.pop();
    match next_id {
      Some(free_key) => {
        EdgeKey {
          idx: free_key.idx,
          generation: free_key.generation + 1
        }
      },
      None => {
        let idx = self.cursor;
        self.cursor = self.cursor + 1;
        EdgeKey {
          idx,
          generation: 0,
        }
      },
    }
  }

  pub fn insert(&mut self, edge: Edge<T>) -> EdgeKey {
    let key = self.get_next_key();
    self.internal_map.insert(key.idx, edge);
    self.keys.insert(key.idx, key);
    key
  }

  pub fn remove(&mut self, key: &EdgeKey) -> Option<Edge<T>> {
    match self.keys.get(&key.idx) {
      Some(found_key) => {
        if key.generation != found_key.generation {
          return None
        }
        self.free_pool.push(key.clone());
        self.internal_map.remove(&key.idx)
      },
      None => None
    }
  }

  pub fn get(&self, key: &EdgeKey) -> Option<&Edge<T>> {
    match self.keys.get(&key.idx) {
      Some(found_key) => {
        if key.generation != found_key.generation {
          return None
        }
        self.internal_map.get(&key.idx)
      },
      None => None
    }
  }

  pub fn has_key(&self, key: &EdgeKey) -> bool {
    match self.keys.get(&key.idx) {
      Some(found_key) => key.generation == found_key.generation,
      None => false
    }
  }
}
