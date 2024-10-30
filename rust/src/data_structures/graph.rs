#[derive(Debug, PartialEq, Eq)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
    pub weight: i32,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

pub struct Graph {
    edges: Vec<Edge>,
    vertices: usize,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        Graph {
            edges: Vec::new(),
            vertices,
        }
    }

    pub fn add_edge(&mut self, source: usize, target: usize, weight: i32) {
        self.edges.push(Edge {
            source,
            target,
            weight,
        });
    }

    pub fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn vertices(&self) -> usize {
        self.vertices
    }
}
