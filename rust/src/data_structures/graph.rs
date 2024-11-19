pub mod adjacency_list;
pub mod edge_list;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeightedEdge<T> {
    pub source: T,
    pub target: T,
    pub weight: usize,
}

impl<T: PartialEq> PartialOrd for WeightedEdge<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.weight.cmp(&other.weight).reverse())
    }
}

impl<T: Eq> Ord for WeightedEdge<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}

pub trait DirectedGraph<T: Clone>: Graph<T> {
    fn add_edge(&mut self, source: T, target: T, weight: usize);
    fn new_random(seed: u64, vertices: Vec<T>, edges: usize, max_weight: usize) -> Self {
        let mut graph = Self::default();
        fastrand::seed(seed);
        for v in 0..vertices.len() {
            graph.add_edge(
                vertices[v].clone(),
                vertices[fastrand::usize(0..vertices.len())].clone(),
                fastrand::usize(0..max_weight),
            );
        }
        for _ in 0..(edges - vertices.len()) {
            let source = vertices[fastrand::usize(0..vertices.len())].clone();
            let target = vertices[fastrand::usize(0..vertices.len())].clone();
            let weight = fastrand::usize(1..max_weight);
            graph.add_edge(source, target, weight);
        }
        graph
    }

    fn new_random_connected(seed: u64, vertices: Vec<T>, edges: usize, max_weight: usize) -> Self {
        let mut graph = Self::default();
        fastrand::seed(seed);
        let mut previous = 0;
        for v in 1..vertices.len() {
            graph.add_edge(
                vertices[previous].clone(),
                vertices[v].clone(),
                fastrand::usize(0..max_weight),
            );
            previous = v;
        }
        for _ in 0..(edges - vertices.len()) {
            let source = vertices[fastrand::usize(0..vertices.len())].clone();
            let target = vertices[fastrand::usize(0..vertices.len())].clone();
            let weight = fastrand::usize(1..max_weight);
            graph.add_edge(source, target, weight);
        }
        graph
    }
}

pub trait UndirectedGraph<T: Clone>: Graph<T> {
    fn add_edge(&mut self, source: T, target: T, weight: usize);
    fn new_random(seed: u64, vertices: Vec<T>, edges: usize, max_weight: usize) -> Self {
        let mut graph = Self::default();
        fastrand::seed(seed);
        for v in 0..vertices.len() {
            graph.add_edge(
                vertices[v].clone(),
                vertices[fastrand::usize(0..vertices.len())].clone(),
                fastrand::usize(0..max_weight),
            );
        }
        for _ in 0..(edges - vertices.len()) {
            let source = vertices[fastrand::usize(0..vertices.len())].clone();
            let target = vertices[fastrand::usize(0..vertices.len())].clone();
            let weight = fastrand::usize(1..max_weight);
            graph.add_edge(source, target, weight);
        }
        graph
    }

    fn new_random_connected(seed: u64, vertices: Vec<T>, edges: usize, max_weight: usize) -> Self {
        let mut graph = Self::default();
        fastrand::seed(seed);
        let mut previous = 0;
        for v in 1..vertices.len() {
            graph.add_edge(
                vertices[previous].clone(),
                vertices[v].clone(),
                fastrand::usize(0..max_weight),
            );
            previous = v;
        }
        for _ in 0..(edges - vertices.len()) {
            let source = vertices[fastrand::usize(0..vertices.len())].clone();
            let target = vertices[fastrand::usize(0..vertices.len())].clone();
            let weight = fastrand::usize(1..max_weight);
            graph.add_edge(source, target, weight);
        }
        graph
    }
}

pub trait Graph<T: Clone>: Default {
    fn nodes(&self) -> Vec<&T>;
    fn neighbors<'a>(&'a self, node: &T) -> impl Iterator<Item = &'a WeightedEdge<T>>
    where
        T: 'a;
    fn size(&self) -> usize;
}

pub trait RandomGraph<T: Clone>: Graph<T> {
    fn new_random(seed: u64, vertices: Vec<T>, edges: usize, max_weight: usize) -> Self;

    fn new_random_connected(seed: u64, vertices: Vec<T>, edges: usize, max_weight: usize) -> Self;
}
