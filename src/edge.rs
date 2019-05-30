use super::traits::NodeID;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum PathDirection {
  Forward,
  Backward
}

#[derive(Debug)]
pub struct EdgePath<T: NodeID> {
  pub from: T::ID_TYPE,
  pub to: T::ID_TYPE,
}

impl<T: NodeID> Clone for EdgePath<T> {
  fn clone(&self) -> Self {
    EdgePath {
      from: self.from.clone(),
      to: self.to.clone(),
    }
  }
}

#[derive(Debug)]
pub struct Edge<T: NodeID> {
  pub forward: EdgePath<T>,
  pub backward: Option<EdgePath<T>>,
}

impl<T: NodeID> Edge<T> {
  pub fn new(from: T::ID_TYPE, to: T::ID_TYPE, bidi: bool) -> Edge<T> {
    let mut backward = None;
    if bidi {
      backward = Some(EdgePath {
        from: to.clone(),
        to: from.clone(),
      })
    }
    let forward = EdgePath {
      from,
      to,
    };
    Edge {
      forward,
      backward
    }
  }

  pub fn get_path(&self, dir: &PathDirection) -> &EdgePath<T> {
    match dir {
      PathDirection::Forward => &self.forward,
      PathDirection::Backward => {
        match &self.backward {
          Some(path) => path,
          _ => panic!()
        }
      }
    }
  }

  pub fn get_connected_id(&self, dir: &PathDirection) -> &<T as NodeID>::ID_TYPE {
    &self.get_path(dir).to
  }

  pub fn is_bidirectional(&self) -> bool {
    self.backward.is_some()
  }
}

impl<T: NodeID> Clone for Edge<T> {
  fn clone(&self) -> Self {
    Edge {
      forward: self.forward.clone(),
      backward: self.backward.clone(),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeKey {
  idx: usize,
  generation: usize,
}

#[derive(Debug)]
pub struct EdgeMap<T: NodeID> {
  cursor: usize,
  free_pool: Vec<EdgeKey>,
  keys: HashMap<usize, EdgeKey>,
  internal_map: HashMap<usize, Edge<T>>
}

impl<T: NodeID> Clone for EdgeMap<T> {
  fn clone(&self) -> Self {
    EdgeMap {
      cursor: self.cursor,
      free_pool: self.free_pool.clone(),
      keys: self.keys.clone(),
      internal_map: self.internal_map.clone()
    }
  }
}

impl<T: NodeID> EdgeMap<T> {
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
