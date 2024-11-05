use algorithms::data_structures::hash_table;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::Instant;

fn main() {
    const capacity: usize = 100000;
    fn probe(key: &String) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.as_str().hash(&mut hasher);
        let hash = hasher.finish() as usize;
        // println!("key: {:?}, hash: {:?}", key, hash);
        1 + (hash % (capacity - 1))
    }

    let tests = capacity as i32;
    let mut chaining_table =
        hash_table::chaining::HashTable::new(capacity).expect("Failed to create a new HashTable");

    let mut double_hashing_table = hash_table::open_address::HashTable::new(capacity, probe)
        .expect("Failed to create a new HashTable");

    let mut linear_probing_table = hash_table::open_address::HashTable::new(capacity, |_| 1)
        .expect("Failed to create a new HashTable");

    //

    let start = Instant::now();
    for i in 0..tests {
        let i = i.to_string();
        chaining_table.insert(i.clone(), i).unwrap();
    }
    let duration = start.elapsed();
    println!("Chaining table insertion time: {:?}", duration);

    let start = Instant::now();
    for i in 0..tests {
        let i = i.to_string();
        chaining_table.get(&i);
    }
    let duration = start.elapsed();
    println!("Chaining table get time: {:?}", duration);

    //

    let start = Instant::now();
    for i in 0..tests {
        let i = i.to_string();
        double_hashing_table.insert(i.clone(), i).unwrap();
    }
    let duration = start.elapsed();
    println!("Chaining table insertion time: {:?}", duration);

    let start = Instant::now();
    for i in 0..tests {
        let i = i.to_string();
        double_hashing_table.get(&i);
    }
    let duration = start.elapsed();
    println!("Chaining table get time: {:?}", duration);

    //

    let start = Instant::now();
    for i in 0..tests {
        let i = i.to_string();
        linear_probing_table.insert(i.clone(), i).unwrap();
    }
    let duration = start.elapsed();
    println!("Chaining table insertion time: {:?}", duration);

    let start = Instant::now();
    for i in 0..tests {
        let i = i.to_string();
        linear_probing_table.get(&i);
    }
    let duration = start.elapsed();
    println!("Chaining table get time: {:?}", duration);
}
