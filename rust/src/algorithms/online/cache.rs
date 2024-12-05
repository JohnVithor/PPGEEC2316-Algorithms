use std::collections::{HashMap, HashSet, VecDeque};

pub trait Cache<K, V> {
    fn get(&mut self, key: K) -> (Option<&V>, usize);
    fn put(&mut self, key: K, value: V);
}

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
}

impl<K: Eq + std::hash::Hash, V> Cache<K, V> for LRUCache<K, V> {
    fn get(&mut self, key: K) -> (Option<&V>, usize) {
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

    fn put(&mut self, key: K, value: V) {
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
        let mut cache: HashSet<K> = HashSet::new();
        let mut to_remove = Vec::new();
        for (i, current) in future_access.iter().enumerate() {
            if !cache.contains(current) {
                if cache.len() >= capacity {
                    let mut dists: HashMap<K, usize> = HashMap::new();

                    for &item in &cache {
                        if let Some(pos) = future_access[i + 1..].iter().position(|&x| x == item) {
                            dists.insert(item, pos);
                        } else {
                            dists.insert(item, usize::MAX);
                        }
                    }

                    if let Some((remove, _)) = dists.iter().max_by_key(|&(_, &dist)| dist) {
                        to_remove.push(Some(*remove));
                        cache.remove(remove);
                    }
                } else {
                    to_remove.push(None);
                }

                cache.insert(*current);
            }
        }
        to_remove
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
}

impl<K: Copy + Eq + std::hash::Hash, V> Cache<K, V> for OracleCache<K, V> {
    fn get(&mut self, key: K) -> (Option<&V>, usize) {
        let mut cost = 0;
        for i in 0..self.capacity {
            if self.cache[i].0 == key {
                return (Some(&self.cache[0].1), cost);
            }
            cost += 1;
        }

        (None, cost)
    }

    fn put(&mut self, key: K, value: V) {
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

pub struct Memory<K, V> {
    cache: Box<dyn Cache<K, V>>,
    data: Vec<(K, V)>,
}
