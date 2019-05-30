pub use gust_macros::NodeID;
use std::hash::Hash;
use std::fmt::Debug;

pub trait NodeID {
  type ID_TYPE: Eq + Hash + Debug + Clone;
}
