use core::hash::Hash;

use crate::data_structures::raw_vec::RawVec;

use super::HashTableError;

pub struct HashTable<K: Hash + PartialEq, V: PartialEq> {
    table: RawVec<Option<(K, V)>>,
    size: usize,
    hasher1: fn(&K) -> usize,
    hasher2: fn(&K) -> usize,
}

impl<K: Hash + PartialEq, V: PartialEq> HashTable<K, V> {
    pub fn new(
        capacity: usize,
        hasher1: fn(&K) -> usize,
        hasher2: fn(&K) -> usize,
    ) -> Result<Self, HashTableError> {
        let mut table = match RawVec::new(capacity) {
            Ok(table) => table,
            Err(e) => return Err(HashTableError::Memory(e)),
        };
        for i in 0..capacity {
            table.set(i, None);
        }
        Ok(HashTable {
            table,
            size: 0,
            hasher1,
            hasher2,
        })
    }

    fn hash(&self, key: &K, i: usize) -> usize {
        ((self.hasher1)(key) + i * (self.hasher2)(key)) % self.size
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), HashTableError> {
        let mut i = 0;
        let best_index = self.hash(&key, i);

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
        let mut index = self.hash(&key, i);
        while index != best_index {
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
            index = self.hash(&key, i);
        }
        Err(HashTableError::Full)
    }

    pub fn remove(&mut self, key: &K) {
        let mut i = 0;
        let best_index = self.hash(key, i);

        if let Some((k, _)) = self.table.get(best_index) {
            if k == key {
                self.table.set(best_index, None);
                return;
            }
        }

        i += 1;
        let mut index = self.hash(key, i);
        while index != best_index {
            if let Some((k, _)) = self.table.get(index) {
                if k == key {
                    self.table.set(best_index, None);
                    return;
                }
            }
            i += 1;
            index = self.hash(key, i);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut i = 0;
        let mut index = self.hash(key, i);
        while let Some((k, v)) = self.table.get(index) {
            if k == key {
                return Some(v);
            }
            i += 1;
            index = self.hash(key, i);
        }
        None
    }
}
