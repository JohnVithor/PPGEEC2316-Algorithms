use std::cmp::Ordering;

use crate::data_structures::graph::Edge;
use crate::data_structures::graph::Graph;

pub fn kruskal(graph: &Graph) -> Vec<&Edge> {
    let mut edges: Vec<&Edge> = graph.edges().iter().collect();
    edges.sort();

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
