use std::collections::{HashMap, HashSet, VecDeque};

pub enum Cache<K: Eq + std::hash::Hash, V: Copy> {
    LRUCache(LRUCache<K, V>),
    OracleCache(OracleCache<K, V>),
    MarkCache(MarkCache<K, V>),
}

impl<K: Eq + std::hash::Hash + Copy, V: Copy> Cache<K, V> {
    fn get(&mut self, key: &K) -> (Option<V>, usize) {
        match self {
            Cache::LRUCache(cache) => cache.get(key),
            Cache::OracleCache(cache) => cache.get(key),
            Cache::MarkCache(cache) => cache.get(key),
        }
    }
    fn put(&mut self, key: K, value: V) {
        match self {
            Cache::LRUCache(cache) => cache.put(key, value),
            Cache::OracleCache(cache) => cache.put(key, value),
            Cache::MarkCache(cache) => cache.put(key, value),
        }
    }
    fn _remove(&mut self, key: &K) {
        match self {
            Cache::LRUCache(cache) => cache.remove(key),
            Cache::OracleCache(cache) => cache.remove(key),
            Cache::MarkCache(cache) => cache.remove(key),
        }
    }
}

pub struct LRUCache<K, V> {
    cache: VecDeque<(K, V)>,
    capacity: usize,
}

impl<K: Eq + std::hash::Hash, V: Copy> From<LRUCache<K, V>> for Cache<K, V> {
    fn from(val: LRUCache<K, V>) -> Self {
        Cache::LRUCache(val)
    }
}

impl<K: Eq + std::hash::Hash, V: Copy> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn get(&mut self, key: &K) -> (Option<V>, usize) {
        let mut cost = 0;
        let mut result = None;
        for (i, (k, _)) in self.cache.iter().enumerate() {
            if *k == *key {
                result = Some(i);
            }
            cost += 1;
        }
        if let Some(i) = result {
            self.cache.swap(i, 0);
            (Some(self.cache[0].1), cost)
        } else {
            (None, cost)
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.cache.len() == self.capacity {
            self.cache.pop_back();
        }
        self.cache.push_front((key, value));
    }

    pub fn remove(&mut self, key: &K) {
        self.cache.retain(|(k, _)| k != key);
    }
}

pub struct OracleCache<K, V> {
    cache: VecDeque<(K, V)>,
    evict_order: Vec<Option<K>>,
    i: usize,
}

impl<K: Eq + std::hash::Hash, V: Copy> From<OracleCache<K, V>> for Cache<K, V> {
    fn from(val: OracleCache<K, V>) -> Self {
        Cache::OracleCache(val)
    }
}

impl<K: Copy + Eq + std::hash::Hash, V: Copy> OracleCache<K, V> {
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
            i: 0,
        }
    }

    pub fn get(&mut self, key: &K) -> (Option<V>, usize) {
        let mut cost = 0;
        for (k, v) in &self.cache {
            if *k == *key {
                return (Some(*v), cost);
            }
            cost += 1;
        }

        (None, cost)
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(Some(evict)) = self.evict_order.get(self.i) {
            for (i, (k, _)) in self.cache.iter().enumerate() {
                if *k == *evict {
                    self.cache.remove(i);
                    self.i += 1;
                    break;
                }
            }
        }
        self.cache.push_front((key, value));
    }

    pub fn remove(&mut self, key: &K) {
        self.cache.retain(|(k, _)| k != key);
    }
}

pub struct MarkCache<K, V> {
    cache: VecDeque<(K, V, bool)>,
    capacity: usize,
}

impl<K: Copy + Eq, V: Copy> MarkCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn get(&mut self, key: &K) -> (Option<V>, usize) {
        let mut cost = 0;
        for (k, v, m) in &mut self.cache {
            if *k == *key {
                *m = true;
                return (Some(*v), cost);
            }
            cost += 1;
        }

        (None, cost)
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.cache.len() == self.capacity {
            let mut marking = true;
            for (_, _, m) in &mut self.cache {
                marking &= *m;
            }
            if marking {
                for (_, _, m) in &mut self.cache {
                    *m = false;
                }
            }
            let options: Vec<usize> = self
                .cache
                .iter()
                .enumerate()
                .filter(|(_, (_, _, m))| !m)
                .map(|(i, _)| i)
                .collect();
            let i = fastrand::usize(0..options.len());

            self.cache[options[i]] = (key, value, true);
        } else {
            self.cache.push_front((key, value, true));
        }
    }

    pub fn remove(&mut self, key: &K) {
        self.cache.retain(|(k, _, _)| k != key);
    }
}

pub struct Memory<K: Copy + Eq + std::hash::Hash, V: Copy> {
    cache: Cache<K, V>,
    data: Vec<(K, V)>,
    access_cost: usize,
}

impl<K: Copy + Eq + std::hash::Hash, V: Copy> Memory<K, V> {
    pub fn new(cache: Cache<K, V>, access_cost: usize) -> Self {
        Self {
            cache,
            data: Vec::new(),
            access_cost,
        }
    }

    pub fn get(&mut self, key: K) -> (Option<V>, usize) {
        let r = self.cache.get(&key);
        let mut cost = r.1;

        if let Some(result) = r.0 {
            (Some(result), cost)
        } else {
            cost += self.access_cost;
            for i in 0..self.data.len() {
                if self.data[i].0 == key {
                    self.cache.put(self.data[i].0, self.data[i].1);
                    return (Some(self.data[i].1), cost);
                }
                cost += self.access_cost;
            }
            (None, cost)
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        self.data.push((key, value));
    }

    pub fn remove(&mut self, key: K) {
        self.data.retain(|(k, _)| k != &key);
    }
}
