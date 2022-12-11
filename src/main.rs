mod toposort;
use toposort::connection::Connection;
use toposort::sort_graph::sort_graph;

fn main() {
    // Test the graph with the following edges:
    // a -> b
    // a -> c

    let mut edges = Vec::new();
    edges.push(Connection {
        src: "a".to_string(),
        dst: "b".to_string(),
    });
    edges.push(Connection {
        src: "a".to_string(),
        dst: "c".to_string(),
    });

    let sorted_graph = sort_graph(&edges);

    // print the linear order
    println!("Linear order:");
    for node in &sorted_graph.linear_order {
        println!("{}", node);
    }
}
