use algorithms::{
    algorithms::graph::topological_ordering::topological_sort,
    data_structures::graph::{adjacency_list::AdjGraph, DirectedGraph},
};

// fn main() {
//     let mut graph = AdjGraph::new(
//         [
//             "undershorts",
//             "pants",
//             "belt",
//             "jacket",
//             "shirt",
//             "tie",
//             "socks",
//             "shoes",
//             "watch",
//         ]
//         .to_vec(),
//     );
//     graph.add_edge("undershorts", "pants", 1);
//     graph.add_edge("pants", "belt", 1);
//     graph.add_edge("pants", "shoes", 1);
//     graph.add_edge("belt", "jacket", 1);
//     graph.add_edge("shirt", "tie", 1);
//     graph.add_edge("shirt", "belt", 1);
//     graph.add_edge("tie", "jacket", 1);
//     graph.add_edge("socks", "shoes", 1);

//     let sorted = topological_sort(&graph);
//     println!("Ordem Topológica: ");
//     for node in sorted {
//         print!("{} ", node);
//     }
//     println!();
// }

fn main() {
    let mut graph = AdjGraph::new(
        [
            "Tree",
            "Binary Tree",
            "Multi-way Tree",
            "Space-Partitioning Tree",
            "Binary Search Tree",
            "Threaded Binary Tree",
            "Rope",
            "Top Tree",
            "Cartesian Tree",
            "Order Statistic Tree",
            "Treap",
            "AVL Tree",
            "Red-Black Tree",
            "Splay Tree",
            "Scapegoat tree",
            "T-Tree",
            "Tango Tree",
            "WAVL Tree",
            "AA Tree",
            "2-3 Tree",
            "2-3-4 Tree",
            "Trie",
            "Fusion Tree",
            "B-Tree",
            "K-D Tree",
            "Quadtree",
            "Octree",
            "Binary Space Partitioning Tree",
            "Maple Tree",
            "B+ Tree",
            "B* Tree",
            "Dancing Tree",
            "Radix Tree",
        ]
        .to_vec(),
    );

    graph.add_edge("Tree", "Binary Tree", 1);
    graph.add_edge("Tree", "Multi-way Tree", 1);
    graph.add_edge("Tree", "Space-Partitioning Tree", 1);
    graph.add_edge("Binary Tree", "Binary Search Tree", 1);
    graph.add_edge("Binary Tree", "Threaded Binary Tree", 1);
    graph.add_edge("Binary Tree", "Rope", 1);
    graph.add_edge("Binary Tree", "Top Tree", 1);
    graph.add_edge("Binary Search Tree", "Cartesian Tree", 1);
    graph.add_edge("Binary Search Tree", "Order Statistic Tree", 1);
    graph.add_edge("Cartesian Tree", "Treap", 1);
    graph.add_edge("Binary Search Tree", "AVL Tree", 1);
    graph.add_edge("Binary Search Tree", "Red-Black Tree", 1);
    graph.add_edge("Binary Search Tree", "Splay Tree", 1);
    graph.add_edge("Binary Search Tree", "Scapegoat tree", 1);
    graph.add_edge("Binary Search Tree", "T-Tree", 1);
    graph.add_edge("Binary Search Tree", "Tango Tree", 1);
    graph.add_edge("Red-Black Tree", "WAVL Tree", 1);
    graph.add_edge("Red-Black Tree", "AA Tree", 1);
    graph.add_edge("AVL Tree", "WAVL Tree", 1);
    graph.add_edge("Multi-way Tree", "2-3 Tree", 1);
    graph.add_edge("Multi-way Tree", "2-3-4 Tree", 1);
    graph.add_edge("Multi-way Tree", "Trie", 1);
    graph.add_edge("Multi-way Tree", "Fusion Tree", 1);
    graph.add_edge("Multi-way Tree", "B-Tree", 1);
    graph.add_edge("Space-Partitioning Tree", "K-D Tree", 1);
    graph.add_edge("Space-Partitioning Tree", "Quadtree", 1);
    graph.add_edge("Space-Partitioning Tree", "Octree", 1);
    graph.add_edge(
        "Space-Partitioning Tree",
        "Binary Space Partitioning Tree",
        1,
    );
    graph.add_edge("Threaded Binary Tree", "Binary Search Tree", 1);
    graph.add_edge("B-Tree", "Maple Tree", 1);
    graph.add_edge("B-Tree", "B+ Tree", 1);
    graph.add_edge("B-Tree", "B* Tree", 1);
    graph.add_edge("B-Tree", "Dancing Tree", 1);
    graph.add_edge("Trie", "Radix Tree", 1);

    let sorted = topological_sort(&graph);
    print!("Ordem Topológica: \"|START|\"");
    for node in sorted {
        print!(" -> {:?}", node);
    }
    println!(" -> \"|END|\"");
}
