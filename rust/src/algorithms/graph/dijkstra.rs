use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::data_structures::graph::Graph;

pub fn dijkstra<T: Clone>(graph: &impl Graph<T>, start: usize, end: usize) -> Option<usize> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    for node in graph.nodes() {
        distances.insert(node, usize::MAX);
    }

    *distances.get_mut(&start).unwrap() = 0;
    heap.push(Node {
        id: start,
        distance: 0,
    });

    while let Some(Node { id, distance }) = heap.pop() {
        if id == end {
            return Some(distance);
        }

        if distance > *distances.get(&id).unwrap() {
            continue;
        }

        for &(neighbor, weight) in graph.get(&id).unwrap() {
            let new_distance = distance + weight;
            if new_distance < *distances.get(&neighbor).unwrap() {
                *distances.get_mut(&neighbor).unwrap() = new_distance;
                heap.push(Node {
                    id: neighbor,
                    distance: new_distance,
                });
            }
        }
    }

    None
}

fn main() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![(1, 4), (7, 8)]);
    graph.insert(1, vec![(0, 4), (2, 8), (7, 11)]);
    graph.insert(2, vec![(1, 8), (3, 7), (5, 4), (8, 2)]);
    graph.insert(3, vec![(2, 7), (4, 9), (5, 14)]);
    graph.insert(4, vec![(3, 9), (5, 10)]);
    graph.insert(5, vec![(2, 4), (3, 14), (4, 10), (6, 2)]);
    graph.insert(6, vec![(5, 2), (7, 1), (8, 6)]);
    graph.insert(7, vec![(0, 8), (1, 11), (6, 1), (8, 7)]);
    graph.insert(8, vec![(2, 2), (6, 6), (7, 7)]);

    let start = 0;
    let end = 8;

    if let Some(distance) = dijkstra(&graph, start, end) {
        println!(
            "Shortest distance from {} to {} is {}",
            start, end, distance
        );
    } else {
        println!("No path found from {} to {}", start, end);
    }
}
