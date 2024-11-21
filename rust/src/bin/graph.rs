use algorithms::data_structures::graph::{UndirectedGraph, WeightedEdge};
use algorithms::{
    algorithms::graph::kruskal::kruskal, data_structures::graph::edge_list::EdgeGraph,
};
use algorithms::{algorithms::graph::prim::prim, data_structures::graph::adjacency_list::AdjGraph};

fn _print_nodes(mst: Vec<&WeightedEdge<usize>>) {
    for edge in mst {
        println!(
            "({}, {}) - weight: {}",
            edge.source, edge.target, edge.weight
        );
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 4 {
        println!("Usage: graph <nodes> <edges> <seed>");
        return;
    }
    let nodes = args[1]
        .parse::<usize>()
        .expect("Invalid number of nodes\ngraph <nodes> <edges> <seed>");
    let edges = args[2]
        .parse::<usize>()
        .expect("Invalid number of edges\ngraph <nodes> <edges> <seed>");
    let seed = args[3]
        .parse::<u64>()
        .expect("Invalid seed\ngraph <nodes> <edges> <seed>");

    let adj_graph = AdjGraph::new_random_connected(seed, (0..nodes).collect(), edges, 100);
    let edge_graph = EdgeGraph::new_random_connected(seed, (0..nodes).collect(), edges, 100);

    let start = std::time::Instant::now();
    let mst = kruskal(&edge_graph);
    let kruskal_edge = start.elapsed().as_nanos();
    let kruskal_edge_cost = mst.iter().map(|x| x.weight).sum::<usize>();

    let start = std::time::Instant::now();
    let mst = prim(&edge_graph);
    let prim_edge = start.elapsed().as_nanos();
    let prim_edge_cost = mst.iter().map(|x| x.weight).sum::<usize>();

    let start = std::time::Instant::now();
    let mst = kruskal(&adj_graph);
    let kruskal_adj = start.elapsed().as_nanos();
    let kruskal_adj_cost = mst.iter().map(|x| x.weight).sum::<usize>();

    let start = std::time::Instant::now();
    let mst = prim(&adj_graph);
    let prim_adj = start.elapsed().as_nanos();
    let prim_adj_cost = mst.iter().map(|x| x.weight).sum::<usize>();

    println!("{kruskal_edge},{prim_edge},{kruskal_adj},{prim_adj},{kruskal_edge_cost},{prim_edge_cost},{kruskal_adj_cost},{prim_adj_cost}")
}
