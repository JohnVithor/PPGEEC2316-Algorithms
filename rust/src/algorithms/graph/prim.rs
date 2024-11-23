use std::{collections::HashSet, hash::Hash};

use crate::data_structures::{
    binary_heap::binary_heap_explicit_key::BinaryHeap,
    graph::{UndirectedGraph, WeightedEdge},
};

pub fn prim<T: Eq + Clone + Hash>(graph: &impl UndirectedGraph<T>) -> Vec<&WeightedEdge<T>> {
    let mut heap = BinaryHeap::new(Vec::new());
    let mut visited = HashSet::new();
    let mut mst: Vec<&WeightedEdge<T>> = Vec::new();
    for i in graph.nodes() {
        if visited.contains(i) {
            continue;
        }
        visited.insert(i);
        for edge in graph.neighbors(i) {
            heap.insert(edge, edge.weight);
        }
        while mst.len() < graph.size() - 1 && !heap.is_empty() {
            let edge = heap.pop().unwrap();
            if visited.contains(&&edge.target) {
                for e in &mut mst {
                    if edge.weight < e.weight
                        && (e.source == edge.source
                            || e.target == edge.target
                            || e.source == edge.target
                            || e.target == edge.source)
                    {
                        *e = edge;
                        break;
                    }
                }
                continue;
            }
            visited.insert(&edge.target);
            let target = &edge.target;
            mst.push(edge);
            for neigh in graph.neighbors(target) {
                if visited.contains(&&neigh.target) {
                    continue;
                }
                heap.insert(neigh, neigh.weight);
            }
        }
    }
    mst
}
