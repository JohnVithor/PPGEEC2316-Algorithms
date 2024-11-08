use std::cmp::Ordering;

use crate::data_structures::{
    binary_heap::binary_heap_explicit_key::BinaryHeap,
    graph::{UndirectedGraph, WeightedEdge},
};

pub fn kruskal(graph: &impl UndirectedGraph) -> Vec<&WeightedEdge> {
    let mut edges: Vec<&WeightedEdge> = {
        let mut edges: Vec<&WeightedEdge> = Vec::new();
        for i in 0..graph.size() {
            for edge in graph.neighbors(i) {
                edges.push(edge);
            }
        }
        edges
    };
    edges.sort_unstable_by(|a, b| a.weight.cmp(&b.weight));

    let mut parent = (0..graph.size()).collect::<Vec<_>>();
    let mut rank = vec![0; graph.size()];

    let mut mst = Vec::new();

    for edge in edges.into_iter() {
        let root_source = find(&mut parent, edge.source);
        let root_target = find(&mut parent, edge.target);

        if root_source != root_target {
            match root_source.cmp(&root_target) {
                Ordering::Less => parent[root_target] = root_source,
                Ordering::Greater => parent[root_source] = root_target,
                Ordering::Equal => {
                    parent[root_target] = root_source;
                    rank[root_source] += 1;
                }
            }
            mst.push(edge);
        }
    }

    mst
}

pub fn find(parent: &mut Vec<usize>, x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

pub fn accidental_kruskal(graph: &impl UndirectedGraph) -> Vec<&WeightedEdge> {
    let mut heap = BinaryHeap::new(vec![]);

    let mut visited = vec![false; graph.size()];

    let mut mst: Vec<&WeightedEdge> = Vec::new();

    for i in 0..graph.size() {
        for edge in graph.neighbors(i) {
            heap.insert(edge, edge.weight);
        }
    }

    while let Some(edge) = heap.pop() {
        if !visited[edge.target] || !visited[edge.source] {
            visited[edge.target] = true;
            visited[edge.source] = true;
            mst.push(edge);
        }
    }
    mst
}
