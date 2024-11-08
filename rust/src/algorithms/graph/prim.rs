use crate::data_structures::{
    binary_heap::binary_heap_explicit_key::BinaryHeap,
    graph::{UndirectedGraph, WeightedEdge},
};

pub fn prim(graph: &impl UndirectedGraph) -> Vec<&WeightedEdge> {
    let mut heap = BinaryHeap::new(vec![]);
    let n = graph.size();
    let mut visited = vec![false; n];
    let mut mst: Vec<&WeightedEdge> = Vec::new();
    for i in 0..n {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        for edge in graph.neighbors(0) {
            heap.insert(edge, edge.weight);
        }
        while let Some(edge) = heap.pop() {
            if visited[edge.target] {
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
            visited[edge.target] = true;
            let target = edge.target;
            mst.push(edge);
            for neigh in graph.neighbors(target) {
                if visited[neigh.target] {
                    continue;
                }
                heap.insert(neigh, neigh.weight);
            }
        }
    }
    mst
}
