//use crate::toposort::dependency_map::DependencyMap;
use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

#[allow(dead_code)]
pub fn has_mark<T: Hash + Clone + Eq>(node: &T, marks: &HashSet<T>) -> bool {
    marks.contains(node)
}

#[allow(dead_code)]
pub fn remove_mark<T: Hash + Clone + Eq>(node: &T, marks: &mut HashSet<T>) {
    marks.remove(node);
}

#[allow(dead_code)]
pub fn add_mark<T: Hash + Clone + Eq>(node: &T, marks: &mut HashSet<T>) {
    marks.insert(node.clone());
}

#[allow(dead_code)]
pub fn get_unmarked_node<T: Hash + Clone + Eq>(
    dependency_map: &HashMap<T, Vec<T>>,
    marks: &HashSet<T>,
) -> Option<T> {
    for node in dependency_map.keys() {
        if !has_mark(node, marks) {
            return Some(node.clone());
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_has_mark() {
        let mut marks = HashSet::new();

        marks.insert("a".to_string());

        assert_eq!(has_mark(&"a".to_string(), &marks), true);
        assert_eq!(has_mark(&"b".to_string(), &marks), false);
    }

    #[test]
    fn test_remove_mark() {
        let mut marks = HashSet::new();
        marks.insert("a".to_string());
        marks.insert("b".to_string());

        assert_eq!(has_mark(&"a".to_string(), &marks), true);
        assert_eq!(has_mark(&"b".to_string(), &marks), true);

        remove_mark(&"a".to_string(), &mut marks);

        assert_eq!(has_mark(&"a".to_string(), &marks), false);
        assert_eq!(has_mark(&"b".to_string(), &marks), true);
    }

    #[test]
    fn test_add_mark() {
        let mut marks = HashSet::new();

        assert_eq!(has_mark(&"a".to_string(), &marks), false);
        assert_eq!(has_mark(&"b".to_string(), &marks), false);

        add_mark(&"a".to_string(), &mut marks);

        assert_eq!(has_mark(&"a".to_string(), &marks), true);
        assert_eq!(has_mark(&"b".to_string(), &marks), false);
    }

    #[test]
    fn test_get_unmarked_node() {
        let mut dependency_map = HashMap::new();
        dependency_map.insert("a".to_string(), vec!["b".to_string()]);
        dependency_map.insert("b".to_string(), vec!["c".to_string()]);
        dependency_map.insert("c".to_string(), vec![]);

        let mut marks = HashSet::new();
        marks.insert("a".to_string());
        marks.insert("b".to_string());

        assert_eq!(
            get_unmarked_node(&dependency_map, &marks),
            Some("c".to_string())
        );

        marks.insert("c".to_string());

        assert_eq!(get_unmarked_node(&dependency_map, &marks), None);
    }
}
