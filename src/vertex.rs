use super::traits::HasID;

#[derive(Debug, Clone, HasID)]
pub struct Vertex {
  #[gust(id)]
  pub id: usize
}

impl Vertex {
  pub fn new(id: usize) -> Vertex {
    Vertex {
      id
    }
  }
}
