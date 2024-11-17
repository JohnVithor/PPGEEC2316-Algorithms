use crate::data_structures::graph::Graph;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Eq, PartialEq, Debug)]
struct Node<T> {
    id: T,
    distance: usize,
}

impl<T: Eq> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<T: Eq> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra<T: Clone + Eq + Hash + Debug>(
    graph: &impl Graph<T>,
    start: &T,
    end: &T,
) -> Option<usize> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    for node in graph.nodes() {
        distances.insert(node, usize::MAX);
    }

    *distances.get_mut(start).unwrap() = 0;
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

        for neigh in graph.neighbors(id) {
            let neighbor = &neigh.target;
            let weight = neigh.weight;

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
