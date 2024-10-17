use core::hash::Hash;

use crate::raw_vec::{RawVec, RawVecError};

pub struct HashTable<K: Hash + PartialEq, V: PartialEq> {
    table: RawVec<Option<(K, V)>>,
    size: usize,
    hasher: fn(&K) -> usize,
}

#[derive(Debug)]
pub enum HashTableError {
    Memory(RawVecError),
    Full,
    Empty,
}

impl<K: Hash + PartialEq, V: PartialEq> HashTable<K, V> {
    pub fn new(size: usize, hasher: fn(&K) -> usize) -> Result<Self, HashTableError> {
        let mut table = match RawVec::new(size) {
            Ok(table) => table,
            Err(e) => return Err(HashTableError::Memory(e)),
        };
        for i in 0..size {
            table.set(i, None);
        }
        Ok(HashTable {
            table,
            size,
            hasher,
        })
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), HashTableError> {
        let best_index = (self.hasher)(&key) % self.size;

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

        let mut index = (best_index + 1) % self.size;
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
            index = (index + 1) % self.size;
        }
        Err(HashTableError::Full)
    }

    pub fn remove(&mut self, key: K) {
        let best_index = (self.hasher)(&key) % self.size;

        if let Some((k, _)) = self.table.get(best_index) {
            if k == &key {
                self.table.set(best_index, None);
                return;
            }
        }

        let mut index = (best_index + 1) % self.size;
        while index != best_index {
            if let Some((k, _)) = self.table.get(index) {
                if k == &key {
                    self.table.set(best_index, None);
                    return;
                }
            }
            index = (index + 1) % self.size;
        }
    }

    pub fn search(&self, key: K) -> Option<&V> {
        let mut index = (self.hasher)(&key) % self.size;
        while let Some((k, v)) = self.table.get(index) {
            if k == &key {
                return Some(v);
            }
            index = (index + 1) % self.size;
        }
        None
    }
}
