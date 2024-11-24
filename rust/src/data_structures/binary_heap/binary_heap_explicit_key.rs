use std::fmt::Debug;

#[derive(Debug)]
pub struct BinaryHeap<T: Eq, K: PartialOrd> {
    pub data: Vec<(K, T)>,
}

impl<T: Eq, K: PartialOrd> BinaryHeap<T, K> {
    pub fn create(data: Vec<T>, keys: Vec<K>) -> Self {
        let mut heap = BinaryHeap {
            data: keys.into_iter().zip(data).collect::<Vec<(K, T)>>(),
        };
        for i in (0..(heap.data.len() / 2)).rev() {
            heap.heapify(i);
        }
        heap
    }

    pub fn new(data: Vec<(K, T)>) -> Self {
        let mut heap = BinaryHeap { data };
        for i in (0..(heap.data.len() / 2)).rev() {
            heap.heapify(i);
        }
        heap
    }

    fn parent(index: usize) -> usize {
        (index.saturating_sub(1)) / 2
    }

    fn left(index: usize) -> usize {
        2 * index + 1
    }

    fn right(index: usize) -> usize {
        2 * index + 2
    }

    fn heapify(&mut self, index: usize) {
        let left = Self::left(index);
        let right = Self::right(index);
        let mut largest = index;

        if left < self.data.len() && self.data[left].0 < self.data[largest].0 {
            largest = left;
        }

        if right < self.data.len() && self.data[right].0 < self.data[largest].0 {
            largest = right;
        }

        if largest != index {
            self.data.swap(index, largest);
            self.heapify(largest);
        }
    }

    pub fn top(&self) -> Option<&T> {
        Some(&self.data.first()?.1)
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        Some(&mut self.data.first_mut()?.1)
    }

    pub fn pop(&mut self) -> Option<(T, K)> {
        if self.data.is_empty() {
            return None;
        }

        let item = self.data.swap_remove(0);
        let max = item.1;
        self.heapify(0);
        Some((max, item.0))
    }

    pub fn update_key(&mut self, value: T, key: K) {
        let index = self.data.iter().position(|x| x.1 == value).unwrap();
        self.data[index].0 = key;
        let mut i = index;
        while i > 0 && self.data[Self::parent(i)].0 > self.data[i].0 {
            self.data.swap(Self::parent(i), i);
            i = Self::parent(i);
        }
    }

    pub fn insert(&mut self, value: T, key: K) {
        self.data.push((key, value));
        let mut i = self.data.len() - 1;
        while i > 0 && self.data[Self::parent(i)].0 > self.data[i].0 {
            self.data.swap(Self::parent(i), i);
            i = Self::parent(i);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get_priority(&self, value: T) -> Option<&K> {
        for (key, val) in self.data.iter() {
            if *val == value {
                return Some(key);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::binary_heap::binary_heap_explicit_key::BinaryHeap;

    #[test]
    fn test_create() {
        let data = vec![1, 2, 3];
        let keys = vec![3, 2, 1];
        let heap = BinaryHeap::create(data, keys);
        assert_eq!(heap.data, vec![(1, 3), (2, 2), (3, 1)]);
    }

    #[test]
    fn test_new() {
        let data = vec![(3, 1), (2, 2), (1, 3)];
        let heap = BinaryHeap::new(data);
        assert_eq!(heap.data, vec![(1, 3), (2, 2), (3, 1)]);
    }

    #[test]
    fn test_top() {
        let data = vec![(3, 1), (2, 2), (1, 3)];
        let heap = BinaryHeap::new(data);
        assert_eq!(heap.top(), Some(&3));
    }

    #[test]
    fn test_top_mut() {
        let data = vec![(3, 1), (2, 2), (1, 3)];
        let mut heap = BinaryHeap::new(data);
        if let Some(top) = heap.top_mut() {
            *top = 4;
        }
        assert_eq!(heap.top(), Some(&4));
    }

    #[test]
    fn test_pop() {
        let data = vec![(3, 1), (2, 2), (1, 3)];
        let mut heap = BinaryHeap::new(data);
        assert_eq!(heap.pop(), Some((3, 1)));
        assert_eq!(heap.data, vec![(2, 2), (3, 1)]);
    }

    #[test]
    fn test_increase_key() {
        let data = vec![(3, 1), (2, 2), (1, 3)];
        let mut heap = BinaryHeap::new(data);
        heap.update_key(2, 4);
        assert_eq!(heap.data, vec![(1, 3), (4, 2), (3, 1)]);
    }

    #[test]
    fn test_insert() {
        let data = vec![(3, 1), (2, 2), (1, 3)];
        let mut heap = BinaryHeap::new(data);
        heap.insert(4, 0);
        assert_eq!(heap.data, vec![(0, 4), (1, 3), (3, 1), (2, 2)]);
    }

    #[test]
    fn test_inserts_ord() {
        let data = vec![];
        let mut heap = BinaryHeap::new(data);
        heap.insert(0, 0);
        heap.insert(1, 1);
        heap.insert(2, 2);
        heap.insert(3, 3);
        heap.insert(4, 4);
        assert_eq!(heap.data, vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)]);
    }

    #[test]
    fn test_inserts_rev() {
        let data = vec![];
        let mut heap = BinaryHeap::new(data);
        heap.insert(4, 4);
        heap.insert(3, 3);
        heap.insert(2, 2);
        heap.insert(1, 1);
        heap.insert(0, 0);
        assert_eq!(heap.data, vec![(0, 0), (1, 1), (3, 3), (4, 4), (2, 2)]);
    }

    #[test]
    fn test_inserts_rev2() {
        let data = vec![];
        let mut heap = BinaryHeap::new(data);
        heap.insert(3, 3);
        assert_eq!(heap.data, vec![(3, 3)]);
        heap.insert(1, 1);
        assert_eq!(heap.data, vec![(1, 1), (3, 3)]);
        heap.insert(2, 2);
        assert_eq!(heap.data, vec![(1, 1), (3, 3), (2, 2)]);
        heap.insert(4, 4);
        assert_eq!(heap.data, vec![(1, 1), (3, 3), (2, 2), (4, 4)]);
        heap.insert(0, 0);
        assert_eq!(heap.data, vec![(0, 0), (1, 1), (2, 2), (4, 4), (3, 3)]);
    }
}
