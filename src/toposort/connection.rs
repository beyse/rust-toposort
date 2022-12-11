use std::clone::Clone;
use std::cmp::Eq;
use std::hash::Hash;

#[allow(dead_code)]
pub struct Connection<T: Hash + Clone + Eq> {
    pub src: T,
    pub dst: T,
}
