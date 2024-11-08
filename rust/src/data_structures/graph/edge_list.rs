use std::collections::HashSet;

use super::{UndirectedGraph, WeightedEdge};

#[derive(Debug, Default)]
pub struct Graph {
    edges: Vec<WeightedEdge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph::default()
    }
}

impl UndirectedGraph for Graph {
    fn add_edge(&mut self, source: usize, target: usize, weight: usize) {
        self.edges.push(WeightedEdge {
            source,
            target,
            weight,
        });
        self.edges.push(WeightedEdge {
            source: target,
            target: source,
            weight,
        });
    }

    fn neighbors(&self, node: usize) -> impl Iterator<Item = &WeightedEdge> {
        self.edges.iter().filter(move |x| x.source == node)
    }
    fn size(&self) -> usize {
        let mut nodes = HashSet::new();
        for edge in &self.edges {
            nodes.insert(edge.source);
            nodes.insert(edge.target);
        }
        nodes.len()
    }
}
