use algorithms::data_structures::hash_table;
use std::env::args;
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::time::Instant;

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
    tests: usize,
) {
    let insertion_start = Instant::now();
    for _ in 0..tests {
        let i = fastrand::usize(0..tests).to_string();
        hashmap.insert(i.clone(), i).unwrap();
    }
    let insertion_duration = insertion_start.elapsed();

    let get_start = Instant::now();
    for _ in 0..tests {
        let i = fastrand::usize(0..tests).to_string();
        hashmap.get(&i);
    }
    let get_duration = get_start.elapsed();
    println!(
        "{name},{:.6},{:.6}",
        get_duration.as_secs_f64(),
        insertion_duration.as_secs_f64()
    );
}

fn eval_hashmap_open(
    hashmap: &mut hash_table::open_address::HashTable<String, String>,
    name: &str,
    tests: usize,
) {
    let insertion_start = Instant::now();
    for _ in 0..tests {
        let i = fastrand::usize(0..tests).to_string();
        hashmap.insert(i.clone(), i).unwrap();
    }
    let insertion_duration = insertion_start.elapsed();

    let get_start = Instant::now();
    for _ in 0..tests {
        let i = fastrand::usize(0..tests).to_string();
        hashmap.get(&i);
    }
    let get_duration = get_start.elapsed();
    println!(
        "{name},{:.6},{:.6}",
        get_duration.as_secs_f64(),
        insertion_duration.as_secs_f64()
    );
}

fn main() {
    let args: Vec<String> = args().collect();
    let seed: u64 = args[1].parse().unwrap();
    let size: usize = args[2].parse().unwrap();
    let modifier: f32 = args[3].parse().unwrap();
    let tests: usize = args[4].parse().unwrap();

    let aug_cap = (size as f32 * modifier) as usize;

    fastrand::seed(seed);

    fn probe(probe_aug_cap: usize, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish() as usize;
        (hash % (probe_aug_cap - 1)) + 1
    }

    let mut chaining_table =
        hash_table::chaining::HashTable::new(size).expect("Failed to create a new HashTable");

    let mut linear_double_hashing_table = hash_table::open_address::HashTable::new(
        aug_cap,
        Box::new(move |key| probe(aug_cap, key as &str)),
        linear,
    )
    .expect("Failed to create a new HashTable");

    let mut linear_probing_table =
        hash_table::open_address::HashTable::new(aug_cap, Box::new(|_| 1), linear)
            .expect("Failed to create a new HashTable");

    let mut square_double_hashing_table = hash_table::open_address::HashTable::new(
        aug_cap,
        Box::new(move |key| probe(aug_cap, key as &str)),
        square,
    )
    .expect("Failed to create a new HashTable");

    let mut square_probing_table =
        hash_table::open_address::HashTable::new(aug_cap, Box::new(|_| 1), square)
            .expect("Failed to create a new HashTable");

    let mut cube_double_hashing_table = hash_table::open_address::HashTable::new(
        aug_cap,
        Box::new(move |key| probe(aug_cap, key as &str)),
        cube,
    )
    .expect("Failed to create a new HashTable");

    let mut cube_probing_table =
        hash_table::open_address::HashTable::new(aug_cap, Box::new(|_| 1), cube)
            .expect("Failed to create a new HashTable");

    //

    eval_hashmap_chain(&mut chaining_table, "chaining_table", tests);

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
    eval_hashmap_open(
        &mut cube_double_hashing_table,
        "cube_double_hashing_table",
        tests,
    );
    eval_hashmap_open(&mut cube_probing_table, "cube_probing_table", tests);
}
