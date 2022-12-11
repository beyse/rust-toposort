//use crate::toposort::dependency_map::DependencyMap;
use crate::toposort::marking::*;
use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

#[allow(dead_code)]
pub fn visit<T: Hash + Clone + Eq>(
    node: &T,
    dependency_map: &HashMap<T, Vec<T>>,
    permanent_marks: &mut HashSet<T>,
    temporary_marks: &mut HashSet<T>,
    sorted_list: &mut VecDeque<T>,
) {
    if has_mark(node, permanent_marks) {
        return;
    }

    if has_mark(node, temporary_marks) {
        panic!("The graph contains a circular dependency. Please make sure the graph is acyclic.");
    }

    add_mark(node, temporary_marks);

    let dependencies = dependency_map.get(node);

    if let Some(dependencies) = dependencies {
        // iterate over all elements in the vector dependencies
        for dependency in dependencies {
            visit(
                dependency,
                dependency_map,
                permanent_marks,
                temporary_marks,
                sorted_list,
            );
        }
    }

    remove_mark(node, temporary_marks);
    add_mark(node, permanent_marks);

    sorted_list.push_front(node.clone());
}

mod test {
    use super::*;

    #[test]
    fn test_visit() {
        let mut dependency_map = HashMap::new();
        dependency_map.insert("a".to_string(), vec!["b".to_string()]);
        dependency_map.insert("b".to_string(), vec!["c".to_string()]);
        dependency_map.insert("c".to_string(), vec![]);

        let mut permanent_marks = HashSet::new();
        let mut temporary_marks = HashSet::new();
        let mut sorted_list = VecDeque::new();

        visit(
            &"a".to_string(),
            &dependency_map,
            &mut permanent_marks,
            &mut temporary_marks,
            &mut sorted_list,
        );

        assert_eq!(permanent_marks.len(), 3);
        assert_eq!(temporary_marks.len(), 0);
        assert_eq!(sorted_list.len(), 3);

        assert_eq!(permanent_marks.contains(&"a".to_string()), true);
        assert_eq!(permanent_marks.contains(&"b".to_string()), true);
        assert_eq!(permanent_marks.contains(&"c".to_string()), true);
    }
}
