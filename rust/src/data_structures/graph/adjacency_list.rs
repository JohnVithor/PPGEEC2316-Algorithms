#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Edge {
    pub target: usize,
    pub weight: i32,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight
            .cmp(&other.weight)
            .then(self.target.cmp(&other.target))
            .reverse()
    }
}

pub struct Graph {
    nodes: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        const ARRAY_REPEAT_VALUE: Vec<Edge> = Vec::new();
        Graph {
            nodes: vec![ARRAY_REPEAT_VALUE; vertices],
        }
    }

    pub fn add_edge(&mut self, source: usize, target: usize, weight: i32) {
        self.nodes[source].push(Edge { target, weight });
        self.nodes[target].push(Edge {
            target: source,
            weight,
        });
    }

    pub fn neighbors(&self, node: usize) -> &Vec<Edge> {
        &self.nodes[node]
    }

    pub fn vertices(&self) -> &Vec<Vec<Edge>> {
        &self.nodes
    }
}
