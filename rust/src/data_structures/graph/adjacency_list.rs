use std::fmt::Debug;

use super::{DirectedGraph, Graph, UndirectedGraph, WeightedEdge};

#[derive(Debug, Default)]
pub struct AdjGraph<T> {
    adjacency: Vec<Vec<WeightedEdge<T>>>,
    nodes: Vec<T>,
}

impl<T> AdjGraph<T> {
    pub fn new(vertices: Vec<T>) -> Self {
        AdjGraph {
            adjacency: vertices.iter().map(|_| Vec::new()).collect(),
            nodes: vertices,
        }
    }

    pub fn vertices(&self) -> &Vec<Vec<WeightedEdge<T>>> {
        &self.adjacency
    }
}

impl<T: PartialEq + Default + Clone> Graph<T> for AdjGraph<T> {
    fn nodes(&self) -> Vec<&T> {
        self.nodes.iter().collect()
    }

    fn neighbors<'a>(&'a self, node: &T) -> impl Iterator<Item = &'a WeightedEdge<T>>
    where
        T: 'a,
    {
        let pos = self.nodes.iter().position(|x| x == node).unwrap();
        self.adjacency[pos].iter()
    }

    fn size(&self) -> usize {
        self.adjacency.len()
    }
}

impl<T: PartialEq + Default + Clone> UndirectedGraph<T> for AdjGraph<T> {
    fn add_edge(&mut self, source: T, target: T, weight: usize) {
        let source_pos = self
            .nodes
            .iter()
            .position(|x| *x == source)
            .or_else(|| {
                self.nodes.push(source.clone());
                self.adjacency.push(Vec::new());
                Some(self.nodes.len() - 1)
            })
            .unwrap();
        let target_pos = self
            .nodes
            .iter()
            .position(|x| *x == target)
            .or_else(|| {
                self.nodes.push(target.clone());
                self.adjacency.push(Vec::new());
                Some(self.nodes.len() - 1)
            })
            .unwrap();
        self.adjacency[source_pos].push(WeightedEdge {
            source: source.clone(),
            target: target.clone(),
            weight,
        });
        self.adjacency[target_pos].push(WeightedEdge {
            target: source,
            source: target,
            weight,
        });
    }
}

impl<T: PartialEq + Default + Clone> DirectedGraph<T> for AdjGraph<T> {
    fn add_edge(&mut self, source: T, target: T, weight: usize) {
        if !self.nodes.contains(&target) {
            self.nodes.push(source.clone());
            self.adjacency.push(Vec::new());
        }
        let source_pos = self
            .nodes
            .iter()
            .position(|x| *x == source)
            .or_else(|| {
                self.nodes.push(target.clone());
                self.adjacency.push(Vec::new());
                Some(self.nodes.len() - 1)
            })
            .unwrap();
        self.adjacency[source_pos].push(WeightedEdge {
            source: source.clone(),
            target: target.clone(),
            weight,
        });
    }
}
