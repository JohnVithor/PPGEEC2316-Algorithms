use super::{fix_capacity, HashTableError};
use crate::data_structures::{linked_list::single::LinkedList, raw_vec::RawVec};
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
pub struct HashTable<K: Hash + PartialEq, V: PartialEq> {
    table: RawVec<LinkedList<(K, V)>>,
    capacity: usize,
}

impl<K: Debug + Hash + PartialEq, V: PartialEq> HashTable<K, V> {
    pub fn new(capacity: usize) -> Result<Self, HashTableError> {
        let capacity = fix_capacity(capacity);
        let mut table = match RawVec::new(capacity) {
            Ok(table) => table,
            Err(e) => return Err(HashTableError::Memory(e)),
        };
        for i in 0..capacity {
            table.set(i, LinkedList::default());
        }
        Ok(HashTable { table, capacity })
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        hash % self.capacity
    }

    pub fn insert(&mut self, key: K, value: V) -> Result<(), HashTableError> {
        let hash = self.hash(&key);
        self.table.get_mut(hash).push_back((key, value));
        Ok(())
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let result = self
            .table
            .get_mut(self.hash(key))
            .remove(|(k, _)| k == key)
            .map(|(_, v)| v);
        result
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash(key);
        self.table
            .get(hash)
            .search(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
}
