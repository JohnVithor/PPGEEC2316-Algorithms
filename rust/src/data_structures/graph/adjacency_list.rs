use super::{UndirectedGraph, WeightedEdge};

#[derive(Debug, Default)]
pub struct Graph {
    nodes: Vec<Vec<WeightedEdge>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        const ARRAY_REPEAT_VALUE: Vec<WeightedEdge> = Vec::new();
        Graph {
            nodes: vec![ARRAY_REPEAT_VALUE; vertices],
        }
    }

    pub fn vertices(&self) -> &Vec<Vec<WeightedEdge>> {
        &self.nodes
    }
}

impl UndirectedGraph for Graph {
    fn add_edge(&mut self, source: usize, target: usize, weight: usize) {
        self.nodes[source].push(WeightedEdge {
            source,
            target,
            weight,
        });
        self.nodes[target].push(WeightedEdge {
            target: source,
            source: target,
            weight,
        });
    }

    fn neighbors(&self, node: usize) -> impl Iterator<Item = &WeightedEdge> {
        self.nodes[node].iter()
    }

    fn size(&self) -> usize {
        self.nodes.len()
    }
}
