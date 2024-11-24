use algorithms::data_structures::graph::{UndirectedGraph, WeightedEdge};
use algorithms::{
    algorithms::graph::kruskal::kruskal, data_structures::graph::edge_list::EdgeGraph,
};
use algorithms::{algorithms::graph::prim::prim, data_structures::graph::adjacency_list::AdjGraph};
use std::fmt::Debug;
use std::hash::Hash;

fn new_random_connected<T: Clone>(
    graph: &mut impl UndirectedGraph<T>,
    seed: u64,
    vertices: Vec<T>,
    edges: usize,
    max_weight: usize,
) {
    fastrand::seed(seed);
    let mut previous = 0;
    for v in 1..vertices.len() {
        graph.add_edge(
            vertices[previous].clone(),
            vertices[v].clone(),
            fastrand::usize(0..max_weight),
        );
        previous = v;
    }

    for _ in 0..(edges - vertices.len()).min(0) {
        let source = vertices[fastrand::usize(0..vertices.len())].clone();
        let target = vertices[fastrand::usize(0..vertices.len())].clone();
        let weight = fastrand::usize(1..max_weight);
        graph.add_edge(source, target, weight);
    }
}

fn new_full_connected<T: Clone>(
    graph: &mut impl UndirectedGraph<T>,
    seed: u64,
    vertices: Vec<T>,
    max_weight: usize,
) {
    fastrand::seed(seed);
    for i in &vertices {
        for j in &vertices {
            let weight = fastrand::usize(1..max_weight);
            graph.add_edge(i.clone(), j.clone(), weight);
        }
    }
}

type EvalResult<'a, T> = (
    (Vec<&'a WeightedEdge<T>>, Vec<WeightedEdge<T>>),
    (u128, u128, usize, usize),
);

fn eval<T: Eq + Clone + Hash + Ord + Clone + Debug>(
    graph: &impl UndirectedGraph<T>,
) -> EvalResult<T> {
    let start = std::time::Instant::now();
    let mst_k = kruskal(graph);
    let kruskal_edge = start.elapsed().as_nanos();
    let kruskal_edge_cost = mst_k.iter().map(|x| x.weight).sum::<usize>();

    let start = std::time::Instant::now();
    let mst_p: Vec<WeightedEdge<T>> = prim(graph);
    let prim_edge = start.elapsed().as_nanos();
    let prim_edge_cost = mst_p.iter().map(|x| x.weight).sum::<usize>();
    (
        (mst_k, mst_p),
        (kruskal_edge, prim_edge, kruskal_edge_cost, prim_edge_cost),
    )
}

fn save_result(nodes: usize, edges: usize, tuple: (u128, u128, usize, usize), modifier: &str) {
    let (kruskal, prim, kruskal_cost, prim_cost) = tuple;
    println!(
        "{},{},{},{},{},{},{}",
        nodes, edges, modifier, kruskal, prim, kruskal_cost, prim_cost
    );
}

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

    let mut adj_graph = AdjGraph::default();
    new_random_connected(&mut adj_graph, seed, (0..nodes).collect(), edges, 100);
    let mut edge_graph = EdgeGraph::default();
    new_random_connected(&mut edge_graph, seed, (0..nodes).collect(), edges, 100);

    let (_msts, r) = eval(&adj_graph);
    save_result(nodes, edges, r, "rand adjacency");
    let (_msts, r) = eval(&edge_graph);
    save_result(nodes, edges, r, "rand edge_list");

    let mut adj_graph = AdjGraph::default();
    new_full_connected(&mut adj_graph, seed, (0..nodes).collect(), 100);
    let mut edge_graph = EdgeGraph::default();
    new_full_connected(&mut edge_graph, seed, (0..nodes).collect(), 100);

    let (_msts, r) = eval(&adj_graph);
    save_result(nodes, nodes * nodes, r, "full adjacency");
    let (_msts, r) = eval(&edge_graph);
    save_result(nodes, nodes * nodes, r, "full edge_list");
}
