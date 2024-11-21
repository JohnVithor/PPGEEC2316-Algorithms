use crate::data_structures::graph::{UndirectedGraph, WeightedEdge};
use std::cmp::Ordering;
use std::fmt::Debug;

pub fn kruskal<T: PartialEq + Clone + Debug>(
    graph: &impl UndirectedGraph<T>,
) -> Vec<&WeightedEdge<T>> {
    let mut edges: Vec<&WeightedEdge<T>> = Vec::new();
    for i in graph.nodes() {
        for edge in graph.neighbors(i) {
            edges.push(edge);
        }
    }
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
