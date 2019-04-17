use super::traits::HasID;

#[derive(Debug)]
pub struct Edge<T, X>
where T: HasID {
  pub from: T::ID_TYPE,
  pub to: T::ID_TYPE,
  pub meta: Option<X>
}
