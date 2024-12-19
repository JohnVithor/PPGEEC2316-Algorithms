use std::env::args;

use algorithms::algorithms::online::cache::{Cache, LRUCache, Memory, OracleCache};

const SIZE: usize = 100;

fn main() -> Result<(), ()> {
    let args: Vec<String> = args().collect();
    if args.len() != 4 {
        println!("Uso: {} <n> <m> <seed>", args[0]);
        return Err(());
    }
    let cache_size: usize = args[1].parse().unwrap();
    let sequence_size: usize = args[2].parse().unwrap();
    let seed: i64 = args[3].parse().unwrap();
    fastrand::seed(seed as u64);

    let access_sequence = (0..sequence_size)
        .map(|_| fastrand::usize(0..SIZE))
        .collect::<Vec<_>>();

    let mut oracle = Memory::new(
        Cache::OracleCache(OracleCache::new(cache_size, access_sequence.clone())),
        100,
    );
    let mut lru = Memory::new(Cache::LRUCache(LRUCache::new(cache_size)), 100);

    for i in 0..SIZE {
        let v = i;
        oracle.put(i, v);
        lru.put(i, v);
    }

    let mut oracle_cost = 0;
    let mut lru_cost = 0;

    let begin = std::time::Instant::now();
    for i in &access_sequence {
        let (_, cost) = oracle.get(*i);
        oracle_cost += cost;
    }
    let oracle_time = begin.elapsed().as_secs_f64();
    let begin = std::time::Instant::now();

    for i in &access_sequence {
        let (_, cost) = lru.get(*i);
        lru_cost += cost;
    }
    let lru_time = begin.elapsed().as_secs_f64();

    println!("{},{},{},{}", oracle_cost, lru_cost, oracle_time, lru_time);
    Ok(())
}
