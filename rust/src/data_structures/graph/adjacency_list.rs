use super::{DirectedGraph, UndirectedGraph, WeightedEdge};

#[derive(Debug, Default)]
pub struct Graph<T> {
    adjacency: Vec<Vec<WeightedEdge<T>>>,
    nodes: Vec<T>,
}

impl<T> Graph<T> {
    pub fn new(vertices: Vec<T>) -> Self {
        Graph {
            adjacency: vertices.iter().map(|_| Vec::new()).collect(),
            nodes: vertices,
        }
    }

    pub fn vertices(&self) -> &Vec<Vec<WeightedEdge<T>>> {
        &self.adjacency
    }
}

impl<T: PartialEq + Default + Clone> UndirectedGraph<T> for Graph<T> {
    fn add_edge(&mut self, source: T, target: T, weight: usize) {
        let source_pos = self.nodes.iter().position(|x| *x == source).unwrap();
        let target_pos = self.nodes.iter().position(|x| *x == target).unwrap();
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

impl<T: PartialEq + Default + Clone> DirectedGraph<T> for Graph<T> {
    fn add_edge(&mut self, source: T, target: T, weight: usize) {
        let source_pos = self.nodes.iter().position(|x| *x == source).unwrap();
        self.adjacency[source_pos].push(WeightedEdge {
            source: source.clone(),
            target: target.clone(),
            weight,
        });
    }

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
