use super::{DirectedGraph, Graph, UndirectedGraph, WeightedEdge};

#[derive(Debug, Default)]
pub struct EdgeGraph<T> {
    edges: Vec<WeightedEdge<T>>,
    nodes: Vec<T>,
}

impl<T: Default> EdgeGraph<T> {
    pub fn new() -> Self {
        EdgeGraph::default()
    }
}

impl<T: Clone + Default + Eq + PartialEq> Graph<T> for EdgeGraph<T> {
    fn nodes(&self) -> Vec<&T> {
        self.nodes.iter().collect()
    }

    fn neighbors<'a>(&'a self, node: &T) -> impl Iterator<Item = &'a WeightedEdge<T>>
    where
        T: 'a,
    {
        self.edges.iter().filter(move |x| x.source == *node)
    }
    fn size(&self) -> usize {
        self.nodes.len()
    }
}

impl<T: Clone + Default + Eq + PartialEq> UndirectedGraph<T> for EdgeGraph<T> {
    fn add_edge(&mut self, source: T, target: T, weight: usize) {
        self.edges.push(WeightedEdge {
            source: source.clone(),
            target: target.clone(),
            weight,
        });
        self.edges.push(WeightedEdge {
            source: target.clone(),
            target: source.clone(),
            weight,
        });
        if self.nodes.contains(&source) {
            self.nodes.push(source);
        }
        if !self.nodes.contains(&target) {
            self.nodes.push(target);
        }
    }
}

impl<T: Clone + Default + Eq + PartialEq> DirectedGraph<T> for EdgeGraph<T> {
    fn add_edge(&mut self, source: T, target: T, weight: usize) {
        self.edges.push(WeightedEdge {
            source: source.clone(),
            target: target.clone(),
            weight,
        });
        if self.nodes.contains(&source) {
            self.nodes.push(source);
        }
        if !self.nodes.contains(&target) {
            self.nodes.push(target);
        }
    }
}
