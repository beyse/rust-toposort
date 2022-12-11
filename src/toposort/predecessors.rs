use crate::toposort::connection::Connection;
//use crate::toposort::dependency_map::DependencyMap;
use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

#[allow(dead_code)]
pub fn count_predecessors<T: Hash + Clone + Eq>(
    node: &T,
    predecessor_map: &HashMap<T, Vec<T>>,
) -> usize {
    let predecessors = predecessor_map.get(node);
    if predecessors.is_none() || predecessors.unwrap().is_empty() {
        return 0;
    }

    let mut max_predecessors = 0;
    for predecessor in predecessors.unwrap() {
        let count = count_predecessors(predecessor, predecessor_map);
        if count > max_predecessors {
            max_predecessors = count;
        }
    }
    max_predecessors + 1
}

#[allow(dead_code)]
/// Creates a map of predecessors for the given edges.
///
/// This function takes a vector of `Connection` values and returns a `DependencyMap`
/// that maps each node in the connections to a vector of its predecessors.
pub fn create_predecessor_map<T: Hash + Clone + Eq>(
    edges: &Vec<Connection<T>>,
) -> HashMap<T, Vec<T>> {
    let mut predecessor_map = HashMap::new();

    for edge in edges {
        let predecessors: &mut Vec<T> = predecessor_map.entry(edge.dst.clone()).or_default();
        predecessors.push(edge.src.clone());
    }

    predecessor_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_predecessors() {
        let mut predecessor_map = HashMap::new();
        predecessor_map.insert("a".to_string(), vec!["b".to_string()]);
        predecessor_map.insert("b".to_string(), vec!["c".to_string()]);
        predecessor_map.insert("c".to_string(), vec![]);

        assert_eq!(count_predecessors(&"a".to_string(), &predecessor_map), 2);
        assert_eq!(count_predecessors(&"b".to_string(), &predecessor_map), 1);
        assert_eq!(count_predecessors(&"c".to_string(), &predecessor_map), 0);
    }
}
