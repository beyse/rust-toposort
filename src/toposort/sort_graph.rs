use crate::toposort::connection::Connection;
use crate::toposort::marking::get_unmarked_node;
use crate::toposort::predecessors::count_predecessors;
use crate::toposort::predecessors::create_predecessor_map;
use crate::toposort::sorted_graph::SortedGraph;
use crate::toposort::visit::visit;
use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

#[allow(dead_code)]
pub fn sort_graph<T: Hash + Clone + Eq>(edges: &Vec<Connection<T>>) -> SortedGraph<T> {
    let mut dependency_map = HashMap::new();

    for edge in edges {
        let dependencies: &mut Vec<T> = dependency_map.entry(edge.src.clone()).or_default();
        dependencies.push(edge.dst.clone());
    }

    let mut permanent_marks = HashSet::new();
    let mut temporary_marks = HashSet::new();

    let mut sorted_list = VecDeque::new();

    let mut node = get_unmarked_node(&dependency_map, &permanent_marks);
    while node.is_some() {
        visit(
            &node.unwrap(),
            &dependency_map,
            &mut permanent_marks,
            &mut temporary_marks,
            &mut sorted_list,
        );
        node = get_unmarked_node(&dependency_map, &permanent_marks);
    }

    let predecessor_map = create_predecessor_map(edges);

    let mut parallel_order: HashMap<usize, Vec<T>> = HashMap::new();
    for node in &sorted_list {
        let predecessors = count_predecessors(node, &predecessor_map);
        let nodes = parallel_order.entry(predecessors).or_default();
        nodes.push(node.clone());
    }

    SortedGraph {
        linear_order: sorted_list.into(),
        parallel_order,
    }
}
