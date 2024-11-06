use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::data_structures::graph::adjacency_list::Graph as AdjGraph;
use crate::data_structures::graph::edge_list::Edge;
use crate::data_structures::graph::edge_list::Graph;

pub fn kruskal(graph: &Graph) -> Vec<&Edge> {
    let mut edges: Vec<&Edge> = graph.edges().iter().collect();
    edges.sort_unstable_by(|e1, e2| e1.cmp(e2).reverse());

    let mut parent = (0..graph.vertices()).collect::<Vec<_>>();
    let mut rank = vec![0; graph.vertices()];

    let mut mst = Vec::new();

    for edge in edges {
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

pub fn accidental_kruskal(graph: &AdjGraph) -> Vec<Edge> {
    let mut heap = BinaryHeap::new();

    let n = graph.vertices().len();

    let mut visited = vec![false; n];

    let mut mst: Vec<Edge> = Vec::new();

    for (source, edges) in graph.vertices().iter().enumerate() {
        for edge in edges {
            heap.push(Edge {
                weight: edge.weight,
                source,
                target: edge.target,
            });
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
