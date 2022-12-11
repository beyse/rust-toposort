use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

#[allow(dead_code)]
/// The sorted graph structure contains the execution order of the graph.
pub struct SortedGraph<T: Hash + Clone + Eq> {
    /// The linear order can be used to execute the graph in a linear fashion.
    pub linear_order: Vec<T>,
    /// The parallel order can be used to execute the graph in parallel.
    /// It contains the nodes that may be executed in parallel.
    pub parallel_order: HashMap<usize, Vec<T>>,
}
