use algorithms::{
    algorithms::graph::topological_ordering::topological_sort,
    data_structures::graph::{adjacency_list::AdjGraph, DirectedGraph},
};

fn main() {
    let mut graph = AdjGraph::new(
        [
            "undershorts",
            "pants",
            "belt",
            "jacket",
            "shirt",
            "tie",
            "socks",
            "shoes",
            "watch",
        ]
        .to_vec(),
    );
    graph.add_edge("undershorts", "pants", 1);
    graph.add_edge("pants", "belt", 1);
    graph.add_edge("pants", "shoes", 1);
    graph.add_edge("belt", "jacket", 1);
    graph.add_edge("shirt", "tie", 1);
    graph.add_edge("shirt", "belt", 1);
    graph.add_edge("tie", "jacket", 1);
    graph.add_edge("socks", "shoes", 1);

    let sorted = topological_sort(&graph);
    println!("Ordem Topológica: ");
    for node in sorted {
        print!("{} ", node);
    }
    println!();
}
