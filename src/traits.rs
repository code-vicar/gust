pub use gust_macros::HasID;
use std::hash::Hash;
use std::fmt::Debug;

pub trait HasID {
  type ID_TYPE: Eq + Hash + Debug + Clone;
  fn get_id(&self) -> &Self::ID_TYPE;
}
