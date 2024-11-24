use crate::data_structures::binary_heap::binary_heap_explicit_key::BinaryHeap;
use crate::data_structures::graph::{UndirectedGraph, WeightedEdge};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;

pub fn kruskal<T: Eq + Clone + Hash + Ord>(
    graph: &impl UndirectedGraph<T>,
) -> Vec<&WeightedEdge<T>> {
    let mut edges = Vec::new();
    for i in graph.nodes() {
        for edge in graph.neighbors(i) {
            edges.push((edge.weight, edge));
        }
    }
    let mut edges = BinaryHeap::new(edges);

    let mut parent: HashMap<&T, &T> = HashMap::new();
    for node in graph.nodes() {
        parent.insert(node, node);
    }
    let mut mst = Vec::new();
    while mst.len() < graph.size() - 1 && !edges.is_empty() {
        let edge: &WeightedEdge<T> = edges.pop().unwrap();
        let root_source = {
            let mut x = &edge.source;
            let mut p = parent[x];
            while p != x {
                x = p;
                p = parent[x];
            }
            p
        };
        let root_target = {
            let mut x = &edge.target;
            let mut p = parent[x];
            while p != x {
                x = p;
                p = parent[x];
            }
            p
        };

        if root_source != root_target {
            match root_source.cmp(root_target) {
                Ordering::Less => parent.insert(root_target, root_source),
                Ordering::Greater => parent.insert(root_source, root_target),
                Ordering::Equal => parent.insert(root_target, root_source),
            };
            mst.push(edge);
        }
    }
    mst
}
