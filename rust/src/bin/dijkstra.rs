use algorithms::{
    algorithms::graph::dijkstra::dijkstra,
    data_structures::graph::{edge_list::EdgeGraph, DirectedGraph},
};

fn main() {
    let mut graph = EdgeGraph::new();
    graph.add_edge(0, 1, 2);
    graph.add_edge(0, 7, 8);
    graph.add_edge(1, 0, 2);
    graph.add_edge(1, 2, 2);
    graph.add_edge(1, 7, 11);
    graph.add_edge(2, 1, 2);
    graph.add_edge(2, 3, 1);
    graph.add_edge(2, 5, 6);
    graph.add_edge(2, 8, 7);
    graph.add_edge(3, 2, 1);
    graph.add_edge(3, 4, 2);
    graph.add_edge(3, 5, 4);
    graph.add_edge(4, 3, 2);
    graph.add_edge(4, 5, 1);
    graph.add_edge(5, 2, 6);
    graph.add_edge(5, 3, 4);
    graph.add_edge(5, 4, 1);
    graph.add_edge(5, 6, 2);
    graph.add_edge(6, 5, 2);
    graph.add_edge(6, 7, 5);
    graph.add_edge(6, 8, 5);
    graph.add_edge(7, 0, 8);
    graph.add_edge(7, 1, 11);
    graph.add_edge(7, 6, 5);
    graph.add_edge(7, 8, 2);
    graph.add_edge(8, 2, 7);
    graph.add_edge(8, 6, 5);
    graph.add_edge(8, 7, 2);

    let start = 0;
    let end = 8;

    if let Some((distance, path)) = dijkstra(&graph, &start, &end) {
        println!(
            "Shortest distance from {} to {} is {} with path {:?}",
            start, end, distance, path
        );
    } else {
        println!("No path found from {} to {}", start, end);
    }
}
