use algorithms::{algorithms::graph::kruskal::kruskal, data_structures::graph::Graph};

fn main() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1, 2);
    graph.add_edge(0, 3, 6);
    graph.add_edge(0, 4, 5);
    graph.add_edge(1, 2, 3);
    graph.add_edge(1, 3, 8);
    graph.add_edge(1, 4, 7);
    graph.add_edge(2, 3, 9);
    graph.add_edge(2, 4, 6);
    graph.add_edge(3, 4, 5);

    let mst = kruskal(&graph);

    println!("Minimum Spanning Tree:");
    for edge in mst {
        println!(
            "({}, {}) - weight: {}",
            edge.source, edge.target, edge.weight
        );
    }
}
