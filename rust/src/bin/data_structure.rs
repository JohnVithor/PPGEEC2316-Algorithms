use algorithms::data_structures::hash_table;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::Instant;

const capacity: usize = 10_000_000;
const modifier: f32 = 2.0;

const aug_cap: usize = (capacity as f32 * modifier) as usize;

fn linear(pos: usize, step: usize) -> usize {
    pos + step
}

fn square(pos: usize, step: usize) -> usize {
    pos + step * step
}

fn cube(pos: usize, step: usize) -> usize {
    pos + step * step * step
}

fn eval_hashmap_chain(
    hashmap: &mut hash_table::chaining::HashTable<String, String>,
    name: &str,
    tests: i32,
) {
    let start = Instant::now();
    for i in 0..tests {
        let i = i.to_string();
        hashmap.insert(i.clone(), i).unwrap();
    }
    let duration = start.elapsed();
    println!("{name} table insertion time: {:?}", duration);

    let start = Instant::now();
    for i in 0..tests {
        let i = i.to_string();
        hashmap.get(&i);
    }
    let duration = start.elapsed();
    println!("{name} table get time: {:?}", duration);
}

fn eval_hashmap_open(
    hashmap: &mut hash_table::open_address::HashTable<String, String>,
    name: &str,
    tests: i32,
) {
    let start = Instant::now();
    for i in 0..tests {
        let i = i.to_string();
        hashmap.insert(i.clone(), i).unwrap();
    }
    let duration = start.elapsed();
    println!("{name} table insertion time: {:?}", duration);

    let start = Instant::now();
    for i in 0..tests {
        let i = i.to_string();
        hashmap.get(&i);
    }
    let duration = start.elapsed();
    println!("{name} table get time: {:?}", duration);
}

fn main() {
    fn probe(key: &String) -> usize {
        let mut hasher = DefaultHasher::new();
        key.as_str().hash(&mut hasher);
        let hash = hasher.finish() as usize;
        (hash % (aug_cap - 1)) + 1
    }

    let tests = capacity as i32;
    let mut chaining_table =
        hash_table::chaining::HashTable::new(capacity).expect("Failed to create a new HashTable");

    let mut linear_double_hashing_table =
        hash_table::open_address::HashTable::new(aug_cap, probe, linear)
            .expect("Failed to create a new HashTable");

    let mut linear_probing_table = hash_table::open_address::HashTable::new(aug_cap, |_| 1, linear)
        .expect("Failed to create a new HashTable");

    let mut square_double_hashing_table =
        hash_table::open_address::HashTable::new(aug_cap, probe, square)
            .expect("Failed to create a new HashTable");

    let mut square_probing_table = hash_table::open_address::HashTable::new(aug_cap, |_| 1, square)
        .expect("Failed to create a new HashTable");

    //

    eval_hashmap_chain(&mut chaining_table, "Chaining", tests);

    eval_hashmap_open(
        &mut linear_double_hashing_table,
        "linear_double_hashing_table",
        tests,
    );

    eval_hashmap_open(&mut linear_probing_table, "linear_probing_table", tests);
    eval_hashmap_open(
        &mut square_double_hashing_table,
        "square_double_hashing_table",
        tests,
    );
    eval_hashmap_open(&mut square_probing_table, "square_probing_table", tests);
}
