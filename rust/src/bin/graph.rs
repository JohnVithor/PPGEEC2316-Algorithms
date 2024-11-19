use algorithms::data_structures::graph::{Graph, UndirectedGraph};
use algorithms::{
    algorithms::graph::kruskal::kruskal, data_structures::graph::edge_list::EdgeGraph,
};
use algorithms::{algorithms::graph::prim::prim, data_structures::graph::adjacency_list::AdjGraph};

fn main() {
    {
        let seed = 0;
        let nodes = 100;
        let edges = 100;

        let adj_graph = AdjGraph::new_random_connected(seed, (0..nodes).collect(), edges, 100);
        let edge_graph = EdgeGraph::new_random_connected(seed, (0..nodes).collect(), edges, 100);
        let start = std::time::Instant::now();
        let mst = kruskal(&edge_graph);
        println!("kruskal elapsed time: {:?}", start.elapsed().as_nanos());

        println!(
            "Minimum Spanning Tree: {}",
            mst.iter().map(|x| x.weight).sum::<usize>()
        );
        // for edge in mst {
        //     println!(
        //         "({}, {}) - weight: {}",
        //         edge.source, edge.target, edge.weight
        //     );
        // }

        let start = std::time::Instant::now();
        let mst = prim(&edge_graph);
        println!("prim elapsed time: {:?}", start.elapsed().as_nanos());

        println!(
            "Minimum Spanning Tree: {}",
            mst.iter().map(|x| x.weight).sum::<usize>()
        );
        // for edge in mst {
        //     println!(
        //         "({}, {}) - weight: {}",
        //         edge.source, edge.target, edge.weight
        //     );
        // }

        let start = std::time::Instant::now();
        let mst = kruskal(&adj_graph);
        println!("kruskal elapsed time: {:?}", start.elapsed().as_nanos());

        println!(
            "Minimum Spanning Tree: {}",
            mst.iter().map(|x| x.weight).sum::<usize>()
        );
        // for edge in mst {
        //     println!(
        //         "({}, {}) - weight: {}",
        //         edge.source, edge.target, edge.weight
        //     );
        // }

        let start = std::time::Instant::now();
        let mst = prim(&adj_graph);
        println!("prim elapsed time: {:?}", start.elapsed().as_nanos());

        println!(
            "Minimum Spanning Tree: {}",
            mst.iter().map(|x| x.weight).sum::<usize>()
        );
        // for edge in mst {
        //     println!(
        //         "({}, {}) - weight: {}",
        //         edge.source, edge.target, edge.weight
        //     );
        // }
    }
}
