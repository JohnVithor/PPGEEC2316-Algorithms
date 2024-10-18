use core::hash::Hash;

use super::{double_hashing::HashTable as DoubleHashingHashTable, HashTableError};

pub struct HashTable<K: Hash + PartialEq, V: PartialEq> {
    table: DoubleHashingHashTable<K, V>,
}

impl<K: Hash + PartialEq, V: PartialEq> HashTable<K, V> {
    pub fn new(capacity: usize, hasher: fn(&K) -> usize) -> Result<Self, HashTableError> {
        Ok(HashTable {
            table: DoubleHashingHashTable::new(capacity, hasher, |_: &K| 1)?,
        })
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), HashTableError> {
        self.table.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.table.get(key)
    }

    pub fn remove(&mut self, key: &K) {
        self.table.remove(key);
    }
}
