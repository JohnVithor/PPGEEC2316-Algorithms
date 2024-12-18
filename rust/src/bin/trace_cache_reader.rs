use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use algorithms::algorithms::online::cache::{Cache, LRUCache, MarkCache, Memory};

fn main() -> Result<(), ()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 4 {
        println!("Uso: {} <path> <cache> <seed> (seed >=0])", args[0],);
        return Err(());
    }
    let path: &str = &args[1];
    let cache_size: usize = args[2].parse().unwrap();
    let seed: i64 = args[2].parse().unwrap();

    if cache_size < 1 || seed < 0 {
        println!("Uso: {} <path> <cache> <seed> (seed >=0])", args[0],);
        return Err(());
    }
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);
    let access_sequence: Vec<usize> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.trim().parse().expect("Could not parse number"))
        .collect();

    fastrand::seed(seed as u64);
    let mut lru_memory = Memory::new(Cache::LRUCache(LRUCache::new(cache_size)), 10);
    let mut mark_memory = Memory::new(Cache::MarkCache(MarkCache::new(cache_size)), 10);

    for &key in &access_sequence {
        lru_memory.put(key, key);
        mark_memory.put(key, key);
    }

    let mut lru_cost = 0;
    let mut mark_cost = 0;

    let now = std::time::Instant::now();
    for &key in &access_sequence {
        let (_, cost) = lru_memory.get(key);
        lru_cost += cost;
    }
    let lru_elapsed = now.elapsed().as_secs_f64();

    let now = std::time::Instant::now();
    for &key in &access_sequence {
        let (_, cost) = mark_memory.get(key);
        mark_cost += cost;
    }
    let mark_elapsed = now.elapsed().as_secs_f64();

    println!(
        "{},{},{},{}",
        lru_cost, mark_cost, lru_elapsed, mark_elapsed
    );

    Ok(())
}
