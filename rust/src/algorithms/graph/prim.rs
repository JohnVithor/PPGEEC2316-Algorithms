use crate::data_structures::{
    binary_heap::binary_heap_explicit_key::BinaryHeap,
    graph::{UndirectedGraph, WeightedEdge},
};

pub fn prim<T: Eq + Clone>(graph: &impl UndirectedGraph<T>) -> Vec<WeightedEdge<T>> {
    let mut heap = BinaryHeap::new(Vec::new());
    let mut mst: Vec<WeightedEdge<T>> = Vec::new();
    let nodes = graph.nodes();
    let mut iter = nodes.into_iter();
    let mut current_node = iter.next().unwrap();
    for node in iter {
        heap.insert(node, usize::MAX);
    }
    for u in graph.neighbors(current_node) {
        if let Some(cost) = heap.get_priority(&u.target) {
            if *current_node != u.target && u.weight < *cost {
                heap.update_key(&u.target, u.weight);
            }
        }
    }
    while !heap.is_empty() {
        let (node, cost) = heap.pop().unwrap();
        mst.push(WeightedEdge {
            source: current_node.clone(),
            target: node.clone(),
            weight: cost,
        });
        for u in graph.neighbors(node) {
            if let Some(cost) = heap.get_priority(&u.target) {
                let new_cost = u.weight;
                if new_cost < *cost {
                    heap.update_key(&u.target, new_cost);
                }
            }
        }
        current_node = node;
    }
    mst
}
