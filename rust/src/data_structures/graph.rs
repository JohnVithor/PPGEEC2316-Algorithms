pub mod adjacency_list;
pub mod edge_list;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeightedEdge {
    pub weight: usize,
    pub source: usize,
    pub target: usize,
}

pub trait UndirectedGraph: Default {
    fn add_edge(&mut self, source: usize, target: usize, weight: usize);
    fn neighbors(&self, node: usize) -> impl Iterator<Item = &WeightedEdge>;
    fn size(&self) -> usize;
    fn random(seed: u64, vertices: usize, edges: usize, max_weight: usize) -> Self {
        let mut graph = Self::default();
        fastrand::seed(seed);
        for v in 0..vertices {
            graph.add_edge(
                v,
                fastrand::usize(0..vertices),
                fastrand::usize(0..max_weight),
            );
        }
        for _ in 0..(edges - vertices) {
            let source = fastrand::usize(0..vertices);
            let target = fastrand::usize(0..vertices);
            let weight = fastrand::usize(1..max_weight);
            graph.add_edge(source, target, weight);
        }
        graph
    }

    fn random_connected(seed: u64, vertices: usize, edges: usize, max_weight: usize) -> Self {
        let mut graph = Self::default();
        fastrand::seed(seed);
        let mut previous = 0;
        for v in 1..vertices {
            graph.add_edge(previous, v, fastrand::usize(0..max_weight));
            previous = v;
        }
        for _ in 0..(edges - vertices) {
            let source = fastrand::usize(0..vertices);
            let target = fastrand::usize(0..vertices);
            let weight = fastrand::usize(1..max_weight);
            graph.add_edge(source, target, weight);
        }
        graph
    }
}
