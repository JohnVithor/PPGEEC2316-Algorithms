pub struct BinaryHeap<T: Eq, K: Ord> {
    pub data: Vec<(K, T)>,
}

impl<T: Eq, K: Ord> BinaryHeap<T, K> {
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
        (index) / 2
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

    pub fn get_max(&self) -> Option<&T> {
        Some(&self.data.first()?.1)
    }

    pub fn get_max_mut(&mut self) -> Option<&mut T> {
        Some(&mut self.data.first_mut()?.1)
    }

    pub fn extract_max(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let max = self.data.swap_remove(0).1;
        self.heapify(0);
        Some(max)
    }

    pub fn increase_key(&mut self, value: &T, key: K) {
        let index = self.data.iter().position(|x| &x.1 == value).unwrap();
        self.data[index].0 = key;
        let mut i = index;
        while i > 0 && self.data[Self::parent(i)].0 < self.data[i].0 {
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
}
