use std::cmp::Ordering;

use crate::data_structures::{
    binary_heap::binary_heap_explicit_key::BinaryHeap,
    graph::{UndirectedGraph, WeightedEdge},
};

pub fn kruskal<T: PartialEq + Clone>(graph: &impl UndirectedGraph<T>) -> Vec<&WeightedEdge<T>> {
    let mut edges: Vec<&WeightedEdge<T>> = {
        let mut edges: Vec<&WeightedEdge<T>> = Vec::new();
        for i in graph.nodes() {
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
        let source = graph
            .nodes()
            .iter()
            .position(|x| *x == &edge.source)
            .unwrap();
        let target = graph
            .nodes()
            .iter()
            .position(|x| *x == &edge.target)
            .unwrap();
        let root_source = find(&mut parent, source);
        let root_target = find(&mut parent, target);

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

pub fn accidental_kruskal<T: Eq + Clone>(graph: &impl UndirectedGraph<T>) -> Vec<&WeightedEdge<T>> {
    let mut heap = BinaryHeap::new(vec![]);

    let mut visited = vec![];

    let mut mst: Vec<&WeightedEdge<T>> = Vec::new();

    for i in graph.nodes() {
        for edge in graph.neighbors(i) {
            heap.insert(edge, edge.weight);
        }
    }

    while let Some(edge) = heap.pop() {
        if !visited.contains(&&edge.target) || !visited.contains(&&edge.source) {
            visited.push(&edge.target);
            visited.push(&edge.source);
            mst.push(edge);
        }
    }
    mst
}
