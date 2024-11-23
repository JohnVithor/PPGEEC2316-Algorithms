use crate::data_structures::graph::DirectedGraph;

pub fn topological_sort<T: PartialEq + Clone>(graph: &impl DirectedGraph<T>) -> Vec<&T> {
    let mut visited = Vec::with_capacity(graph.size());
    let mut result = Vec::with_capacity(graph.size());

    for node in graph.nodes() {
        if !visited.contains(&node) {
            dfs(node, graph, &mut visited, &mut result);
        }
    }
    result.reverse();
    result
}

fn dfs<'a, T: PartialEq + Clone>(
    node: &'a T,
    graph: &'a impl DirectedGraph<T>,
    visited: &mut Vec<&'a T>,
    result: &mut Vec<&'a T>,
) {
    visited.push(node);

    for neighbor in graph.neighbors(node) {
        if !visited.contains(&&(neighbor.target)) {
            dfs(&neighbor.target, graph, visited, result);
        }
    }

    result.push(node);
}
