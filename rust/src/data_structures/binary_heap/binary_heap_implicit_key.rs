use std::fmt::Debug;

pub struct BinaryHeap<T: Ord> {
    pub data: Vec<T>,
}

impl<T: Ord + Debug> BinaryHeap<T> {
    pub fn new(data: Vec<T>) -> Self {
        let mut heap = BinaryHeap { data };
        for i in (0..(heap.data.len() / 2)).rev() {
            heap.max_heapify(i);
        }
        heap
    }

    fn parent(index: usize) -> usize {
        (index) / 2
    }

    fn left(index: usize) -> usize {
        2 * index + 1
    }

    fn right(index: usize) -> usize {
        2 * index + 2
    }

    fn max_heapify(&mut self, index: usize) {
        let left = Self::left(index);
        let right = Self::right(index);
        let mut largest = index;

        if left < self.data.len() && self.data[left] > self.data[largest] {
            largest = left;
        }

        if right < self.data.len() && self.data[right] > self.data[largest] {
            largest = right;
        }

        if largest != index {
            self.data.swap(index, largest);
            self.max_heapify(largest);
        }
    }

    pub fn get_max(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn extract_max(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let max = self.data.swap_remove(0);
        self.max_heapify(0);
        Some(max)
    }

    pub fn insert(&mut self, value: T) {
        self.data.push(value);
        let mut i = self.data.len() - 1;
        while i > 0 && self.data[Self::parent(i)] < self.data[i] {
            self.data.swap(Self::parent(i), i);
            i = Self::parent(i);
        }
    }
}
