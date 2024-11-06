use algorithms::algorithms::graph::kruskal::accidental_kruskal;
use algorithms::{
    algorithms::graph::kruskal::kruskal, data_structures::graph::edge_list::Graph as EdgeGraph,
};
use algorithms::{
    algorithms::graph::prim::prim, data_structures::graph::adjacency_list::Graph as AdjGraph,
};

fn main() {
    let mut graph = EdgeGraph::new(5);
    graph.add_edge(0, 1, 2);
    graph.add_edge(0, 3, 6);
    graph.add_edge(0, 4, 5);
    graph.add_edge(1, 2, 3);
    graph.add_edge(1, 4, 7);
    graph.add_edge(2, 0, 1);
    graph.add_edge(2, 3, 9);
    graph.add_edge(2, 4, 6);
    graph.add_edge(3, 4, 5);
    graph.add_edge(3, 1, 5);

    let start = std::time::Instant::now();
    let mst = kruskal(&graph);
    println!("kruskal elapsed time: {:?}", start.elapsed());

    println!("Minimum Spanning Tree:");
    for edge in mst {
        println!(
            "({}, {}) - weight: {}",
            edge.source, edge.target, edge.weight
        );
    }

    let mut graph = AdjGraph::new(5);
    graph.add_edge(0, 1, 2);
    graph.add_edge(0, 3, 6);
    graph.add_edge(0, 4, 5);
    graph.add_edge(1, 2, 3);
    graph.add_edge(1, 4, 7);
    graph.add_edge(2, 0, 1);
    graph.add_edge(2, 3, 9);
    graph.add_edge(2, 4, 6);
    graph.add_edge(3, 4, 5);
    graph.add_edge(3, 1, 5);

    let start = std::time::Instant::now();
    let mst = prim(&graph);
    println!("prim elapsed time: {:?}", start.elapsed());

    println!("Minimum Spanning Tree:");
    for edge in mst {
        println!(
            "({}, {}) - weight: {}",
            edge.source, edge.target, edge.weight
        );
    }

    let start = std::time::Instant::now();
    let mst = accidental_kruskal(&graph);
    println!("accidental kruskal elapsed time: {:?}", start.elapsed());

    println!("Minimum Spanning Tree:");
    for edge in mst {
        println!(
            "({}, {}) - weight: {}",
            edge.source, edge.target, edge.weight
        );
    }
}
