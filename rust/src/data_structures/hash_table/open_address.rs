use super::HashTableError;
use crate::data_structures::raw_vec::RawVec;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};

pub struct HashTable<K: Hash + PartialEq, V: PartialEq> {
    table: RawVec<Option<(K, V)>>,
    capacity: usize,
    probe: fn(&K) -> usize,
}

impl<K: Debug + Hash + PartialEq, V: PartialEq> HashTable<K, V> {
    pub fn new(capacity: usize, probe: fn(&K) -> usize) -> Result<Self, HashTableError> {
        let mut table = match RawVec::new(capacity) {
            Ok(table) => table,
            Err(e) => return Err(HashTableError::Memory(e)),
        };
        for i in 0..capacity {
            table.set(i, None);
        }
        Ok(HashTable {
            table,
            capacity,
            probe,
        })
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.capacity
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), HashTableError> {
        let mut modifier = 0;
        let hash = self.hash(&key);
        let step = (self.probe)(&key);
        let mut i = 0;
        let mut best_index = (modifier + hash + i * step) % self.capacity;
        match self.table.get(best_index) {
            Some((k, v)) => {
                if k == &key && v == &value {
                    return Ok(());
                }
            }
            None => {
                self.table.set(best_index, Some((key, value)));
                return Ok(());
            }
        }
        i += 1;
        let mut index = (modifier + hash + i * step) % self.capacity;
        while modifier < self.capacity {
            match self.table.get(index) {
                Some((k, v)) => {
                    if k == &key && v == &value {
                        return Ok(());
                    }
                }
                None => {
                    self.table.set(index, Some((key, value)));
                    return Ok(());
                }
            }
            i += 1;
            index = (modifier + hash + i * step) % self.capacity;
            if index == best_index {
                modifier += 1;
                best_index = (modifier + hash + i * step) % self.capacity;
                index = best_index;
            }
        }
        Err(HashTableError::Full)
    }

    pub fn remove(&mut self, key: &K) {
        let mut modifier = 0;
        let hash = self.hash(key);
        let step = (self.probe)(key);
        let mut i = 0;
        let mut best_index = (modifier + hash + i * step) % self.capacity;

        if let Some((k, _)) = self.table.get(best_index) {
            if k == key {
                self.table.set(best_index, None);
                return;
            }
        }

        i += 1;
        let mut index = (modifier + hash + i * step) % self.capacity;
        while modifier < self.capacity {
            if let Some((k, _)) = self.table.get(index) {
                if k == key {
                    self.table.set(best_index, None);
                    return;
                }
            }
            i += 1;
            index = (modifier + hash + i * step) % self.capacity;
            if index == best_index {
                modifier += 1;
                best_index = (modifier + hash + i * step) % self.capacity;
                index = best_index;
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut modifier = 0;
        let hash = self.hash(key);
        let step = (self.probe)(key);
        let mut i = 0;
        let mut index = (modifier + hash + i * step) % self.capacity;
        let mut best_index = index;
        while modifier < self.capacity {
            let (k, v) = self.table.get(index).as_ref().unwrap();
            if k == key {
                return Some(v);
            }
            i += 1;
            index = (modifier + hash + i * step) % self.capacity;
            if index == best_index {
                modifier += 1;
                best_index = (modifier + hash + i * step) % self.capacity;
                index = best_index;
            }
        }
        None
    }
}
