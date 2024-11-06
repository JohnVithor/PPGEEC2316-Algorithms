use std::collections::BinaryHeap;

use crate::data_structures::graph::adjacency_list::Graph;
use crate::data_structures::graph::edge_list::Edge as FullEdge;

pub fn prim(graph: &Graph) -> Vec<FullEdge> {
    let mut heap = BinaryHeap::new();
    let n = graph.vertices().len();
    let mut visited = vec![false; n];
    let mut mst: Vec<FullEdge> = Vec::new();

    visited[0] = true;
    for edge in graph.vertices()[0].iter() {
        heap.push(FullEdge {
            weight: edge.weight,
            source: 0,
            target: edge.target,
        });
    }

    while let Some(edge) = heap.pop() {
        if !visited[edge.target] {
            visited[edge.target] = true;
            let target = edge.target;
            mst.push(edge);
            for neigh in graph.vertices()[target].iter() {
                if !visited[neigh.target] {
                    heap.push(FullEdge {
                        weight: neigh.weight,
                        source: target,
                        target: neigh.target,
                    });
                }
            }
        }
    }

    mst
}
