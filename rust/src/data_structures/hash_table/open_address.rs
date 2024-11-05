use super::{fix_capacity, HashTableError};
use crate::data_structures::raw_vec::RawVec;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};

pub struct HashTable<K: Hash + PartialEq, V: PartialEq> {
    table: RawVec<Option<(K, V)>>,
    capacity: usize,
    probe: fn(&K) -> usize,
    steper: fn(usize, usize) -> usize,
}

impl<K: Debug + Hash + PartialEq, V: PartialEq> HashTable<K, V> {
    pub fn new(
        capacity: usize,
        probe: fn(&K) -> usize,
        steper: fn(usize, usize) -> usize,
    ) -> Result<Self, HashTableError> {
        let capacity = fix_capacity(capacity);
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
            steper,
        })
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.capacity
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), HashTableError> {
        let hash = self.hash(&key);
        let step = (self.probe)(&key);
        let mut index = hash % self.capacity;
        let initial_index = index;
        while let Some((k, _)) = self.table.get(index).as_ref() {
            if k == &key {
                self.table.set(index, Some((key, value)));
                return Ok(());
            }
            index = (self.steper)(index, step) % self.capacity;
            if index == initial_index {
                return Err(HashTableError::Full);
            }
        }
        self.table.set(index, Some((key, value)));
        Ok(())
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let hash = self.hash(key);
        let step = (self.probe)(key);
        let mut index = hash % self.capacity;
        let initial_index = index;
        while let Some((k, _)) = self.table.get(index).as_ref() {
            if k == key {
                let r = self.table.get_mut(index).take();
                self.table.set(index, None);
                return r.map(|(_, v)| v);
            }
            index = (self.steper)(index, step) % self.capacity;
            if index == initial_index {
                break;
            }
        }
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash(key);
        let step = (self.probe)(key);
        let mut index = hash % self.capacity;
        let initial_index = index;
        while let Some((k, v)) = self.table.get(index).as_ref() {
            if k == key {
                return Some(v);
            }
            index = (self.steper)(index, step) % self.capacity;
            if index == initial_index {
                break;
            }
        }
        None
    }
}
