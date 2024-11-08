use algorithms::algorithms::graph::kruskal::accidental_kruskal;
use algorithms::data_structures::graph::UndirectedGraph;
use algorithms::{
    algorithms::graph::kruskal::kruskal, data_structures::graph::edge_list::Graph as EdgeGraph,
};
use algorithms::{
    algorithms::graph::prim::prim, data_structures::graph::adjacency_list::Graph as AdjGraph,
};

fn main() {
    {
        let mut edge_graph = EdgeGraph::new();
        edge_graph.add_edge(0, 1, 2);
        edge_graph.add_edge(0, 3, 6);
        edge_graph.add_edge(0, 4, 5);
        edge_graph.add_edge(1, 2, 3);
        edge_graph.add_edge(1, 4, 7);
        edge_graph.add_edge(2, 0, 1);
        edge_graph.add_edge(2, 3, 9);
        edge_graph.add_edge(2, 4, 6);
        edge_graph.add_edge(3, 4, 5);
        edge_graph.add_edge(3, 1, 5);

        let start = std::time::Instant::now();
        let mst = kruskal(&edge_graph);
        println!("kruskal elapsed time: {:?}", start.elapsed().as_nanos());

        println!(
            "Minimum Spanning Tree: {}",
            mst.iter().map(|x| x.weight).sum::<usize>()
        );

        let start = std::time::Instant::now();
        let mst = prim(&edge_graph);
        println!("prim elapsed time: {:?}", start.elapsed().as_nanos());

        println!(
            "Minimum Spanning Tree: {}",
            mst.iter().map(|x| x.weight).sum::<usize>()
        );

        let mut adj_graph = AdjGraph::new(5);
        adj_graph.add_edge(0, 1, 2);
        adj_graph.add_edge(0, 3, 6);
        adj_graph.add_edge(0, 4, 5);
        adj_graph.add_edge(1, 2, 3);
        adj_graph.add_edge(1, 4, 7);
        adj_graph.add_edge(2, 0, 1);
        adj_graph.add_edge(2, 3, 9);
        adj_graph.add_edge(2, 4, 6);
        adj_graph.add_edge(3, 4, 5);
        adj_graph.add_edge(3, 1, 5);

        let start = std::time::Instant::now();
        let mst = kruskal(&adj_graph);
        println!("kruskal elapsed time: {:?}", start.elapsed().as_nanos());

        println!(
            "Minimum Spanning Tree: {}",
            mst.iter().map(|x| x.weight).sum::<usize>()
        );

        let start = std::time::Instant::now();
        let mst = prim(&adj_graph);
        println!("prim elapsed time: {:?}", start.elapsed().as_nanos());

        println!(
            "Minimum Spanning Tree: {}",
            mst.iter().map(|x| x.weight).sum::<usize>()
        );
    }
    {
        let g1 = EdgeGraph::random_connected(0, 12, 100, 100);
        let start = std::time::Instant::now();
        let kruskal_mst = accidental_kruskal(&g1);
        println!("kruskal elapsed time: {:?}", start.elapsed().as_nanos());
        println!(
            "Minimum Spanning Tree: {}",
            kruskal_mst.iter().map(|x| x.weight).sum::<usize>()
        );
        for edge in kruskal_mst {
            println!(
                "({}, {}) - weight: {}",
                edge.source, edge.target, edge.weight
            );
        }

        let start = std::time::Instant::now();
        let prim_mst = prim(&g1);
        println!("prim elapsed time: {:?}", start.elapsed().as_nanos());
        println!(
            "Minimum Spanning Tree: {}",
            prim_mst.iter().map(|x| x.weight).sum::<usize>()
        );
        for edge in prim_mst {
            println!(
                "({}, {}) - weight: {}",
                edge.source, edge.target, edge.weight
            );
        }
    }
}
