use std::collections::VecDeque;

pub struct LRUCache<K, V> {
    cache: VecDeque<(K, V)>,
    capacity: usize,
}

impl<K: Eq + std::hash::Hash, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn get(&mut self, key: K) -> (Option<&V>, usize) {
        let mut cost = 0;
        for i in 0..self.capacity {
            if self.cache[i].0 == key {
                self.cache.swap(i, 0);
                return (Some(&self.cache[0].1), cost);
            }
            cost += 1;
        }
        (None, cost)
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.cache.len() == self.capacity {
            self.cache.pop_back();
        }
        self.cache.push_front((key, value));
    }
}

pub struct OracleCache<K, V> {
    cache: VecDeque<(K, V)>,
    evict_order: Vec<Option<K>>,
    capacity: usize,
    i: usize,
}

impl<K: Copy + Eq + std::hash::Hash, V> OracleCache<K, V> {
    fn calculate(future_access: Vec<K>, capacity: usize) -> Vec<Option<K>> {
        // let mut evict_order = vec![None; capacity];
        // let mut future_access = future_access.into_iter().rev();
        // for i in 0..capacity {
        //     evict_order[i] = future_access.next();
        // }
        // evict_order
        todo!("Implement this function");
    }

    pub fn new(capacity: usize, future_access: Vec<K>) -> Self {
        let evict_order = Self::calculate(future_access, capacity);
        Self {
            cache: VecDeque::with_capacity(capacity),
            evict_order,
            capacity,
            i: 0,
        }
    }

    pub fn get(&mut self, key: K) -> (Option<&V>, usize) {
        let mut cost = 0;
        for i in 0..self.capacity {
            if self.cache[i].0 == key {
                return (Some(&self.cache[0].1), cost);
            }
            cost += 1;
        }

        (None, cost)
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(Some(evict)) = self.evict_order.get(self.i) {
            for i in 0..self.capacity {
                if self.cache[i].0 == *evict {
                    self.cache.remove(i);
                    self.i += 1;
                    break;
                }
            }
        }
        self.cache.push_front((key, value));
    }
}
