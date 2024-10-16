use std::fmt::Debug;

pub struct BinaryHeap<T> {
    pub data: Vec<(f32, T)>,
}

impl<T: Ord + Debug> BinaryHeap<T> {
    pub fn new(data: Vec<T>, keys: Vec<f32>) -> Self {
        let mut heap = BinaryHeap {
            data: keys.into_iter().zip(data).collect::<Vec<(f32, T)>>(),
        };
        for i in (0..(heap.data.len() / 2)).rev() {
            heap.max_heapfy(i);
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

    fn max_heapfy(&mut self, index: usize) {
        let left = Self::left(index);
        let right = Self::right(index);
        let mut largest = index;

        if left < self.data.len() && self.data[left].0 > self.data[largest].0 {
            largest = left;
        }

        if right < self.data.len() && self.data[right].0 > self.data[largest].0 {
            largest = right;
        }

        if largest != index {
            self.data.swap(index, largest);
            self.max_heapfy(largest);
        }
    }

    pub fn get_max(&self) -> Option<&T> {
        Some(&self.data.first()?.1)
    }

    pub fn extract_max(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let max = self.data.swap_remove(0).1;
        self.max_heapfy(0);
        Some(max)
    }

    pub fn increase_key(&mut self, value: &T, key: f32) {
        let index = self.data.iter().position(|x| &x.1 == value).unwrap();
        self.data[index].0 = key;
        let mut i = index;
        while i > 0 && self.data[Self::parent(i)].0 < self.data[i].0 {
            self.data.swap(Self::parent(i), i);
            i = Self::parent(i);
        }
    }

    pub fn insert(&mut self, value: T, key: f32) {
        self.data.push((key, value));
        let mut i = self.data.len() - 1;
        while i > 0 && self.data[Self::parent(i)] < self.data[i] {
            self.data.swap(Self::parent(i), i);
            i = Self::parent(i);
        }
    }
}
