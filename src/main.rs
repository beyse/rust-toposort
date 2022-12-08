use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type NodeIdType = String; // This is not super necessary, but it explains the code better.
type DependencyMap = HashMap<NodeIdType, Vec<NodeIdType>>;

struct SortedGraph {
    linear_order: Vec<NodeIdType>,
    parallel_order: HashMap<usize, Vec<NodeIdType>>,
    dependency_map: DependencyMap,
}

struct Connection {
    src: NodeIdType,
    dst: NodeIdType,
}

fn has_mark(node: &NodeIdType, marks: &HashSet<NodeIdType>) -> bool {
    marks.contains(node)
}

fn remove_mark(node: &NodeIdType, marks: &mut HashSet<NodeIdType>) {
    marks.remove(node);
}

fn add_mark(node: &NodeIdType, marks: &mut HashSet<NodeIdType>) {
    marks.insert(node.clone());
}

fn get_unmarked_node(
    dependency_map: &DependencyMap,
    marks: &HashSet<NodeIdType>,
) -> Option<NodeIdType> {
    for (node, dependencies) in dependency_map {
        if !has_mark(node, marks) {
            return Some(node.clone());
        }
    }
    None
}

fn visit(
    node: &NodeIdType,
    dependency_map: &DependencyMap,
    permanent_marks: &mut HashSet<NodeIdType>,
    temporary_marks: &mut HashSet<NodeIdType>,
    sorted_list: &mut VecDeque<NodeIdType>,
) {
    if has_mark(node, permanent_marks) {
        return;
    }

    if has_mark(node, temporary_marks) {
        panic!("The graph contains a circular dependency. Please make sure the graph is acyclic.");
    }

    add_mark(node, temporary_marks);

    let dependencies = dependency_map.get(node);

    if dependencies.is_some() {
        // iterate over all elements in the vector dependencies
        for dependency in dependencies.unwrap() {
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

fn count_predecessors(node: &NodeIdType, predecessor_map: &DependencyMap) -> usize {
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

fn create_predecessor_map(edges: &Vec<Connection>) -> DependencyMap {
    let mut predecessor_map = HashMap::new();

    for edge in edges {
        let predecessors = predecessor_map.entry(edge.dst.clone()).or_insert(vec![]);
        predecessors.push(edge.src.clone());
    }

    predecessor_map
}

fn sort_graph(edges: &Vec<Connection>) -> SortedGraph {
    let mut dependency_map = HashMap::new();

    for edge in edges {
        let dependencies = dependency_map.entry(edge.src.clone()).or_insert(vec![]);
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

    let mut parallel_order: HashMap<usize, Vec<NodeIdType>> = HashMap::new();
    for node in &sorted_list {
        let predecessors = count_predecessors(&node, &predecessor_map);
        let nodes = parallel_order.entry(predecessors).or_insert(vec![]);
        nodes.push(node.clone());
    }

    SortedGraph {
        linear_order: sorted_list.into(),
        parallel_order,
        dependency_map,
    }
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

    #[test]
    fn test_sort_graph() {
        let mut edges = Vec::new();

        // Test the graph with the following edges:
        // a -> b
        // a -> c
        // a -> d
        // b -> d
        // c -> d
        // d -> e
        // c -> e
        // a -> e
        edges.push(Connection {
            src: "a".to_string(),
            dst: "b".to_string(),
        });
        edges.push(Connection {
            src: "a".to_string(),
            dst: "c".to_string(),
        });
        edges.push(Connection {
            src: "a".to_string(),
            dst: "d".to_string(),
        });
        edges.push(Connection {
            src: "b".to_string(),
            dst: "d".to_string(),
        });
        edges.push(Connection {
            src: "c".to_string(),
            dst: "d".to_string(),
        });
        edges.push(Connection {
            src: "d".to_string(),
            dst: "e".to_string(),
        });
        edges.push(Connection {
            src: "c".to_string(),
            dst: "e".to_string(),
        });
        edges.push(Connection {
            src: "a".to_string(),
            dst: "e".to_string(),
        });

        let sorted_graph = sort_graph(&edges);

        // We expect the following linear order:
        // a, b, c, d, e
        // or
        // a, c, b, d, e

        assert_eq!(sorted_graph.linear_order.len(), 5);

        assert_eq!(sorted_graph.linear_order[0], "a".to_string());
        if sorted_graph.linear_order[1] == "b".to_string() {
            assert_eq!(sorted_graph.linear_order[1], "b".to_string());
            assert_eq!(sorted_graph.linear_order[2], "c".to_string());
        } else {
            assert_eq!(sorted_graph.linear_order[1], "c".to_string());
            assert_eq!(sorted_graph.linear_order[2], "b".to_string());
        }
        assert_eq!(sorted_graph.linear_order[3], "d".to_string());
        assert_eq!(sorted_graph.linear_order[4], "e".to_string());

        // print length of parallel order
        println!(
            "parallel order length: {}",
            sorted_graph.parallel_order.len()
        );

        // print the parallel order
        for (key, value) in &sorted_graph.parallel_order {
            println!("{}: {:?}", key, value);
        }

        // we expect the following parallel order:
        // 0 -> a
        // 1 -> c, b (or b, c)
        // 2 -> d
        // 3 -> e
        assert_eq!(sorted_graph.parallel_order.len(), 4);
        assert_eq!(sorted_graph.parallel_order[&0].len(), 1);
        assert_eq!(sorted_graph.parallel_order[&1].len(), 2);
        assert_eq!(sorted_graph.parallel_order[&2].len(), 1);
        assert_eq!(sorted_graph.parallel_order[&3].len(), 1);

        assert!(sorted_graph.parallel_order[&0].contains(&"a".to_string()));

        assert!(sorted_graph.parallel_order[&1].contains(&"c".to_string()));
        assert!(sorted_graph.parallel_order[&1].contains(&"b".to_string()));

        assert!(sorted_graph.parallel_order[&2].contains(&"d".to_string()));

        assert!(sorted_graph.parallel_order[&3].contains(&"e".to_string()));
    }
}

fn main() {
    println!("Hey");
}
