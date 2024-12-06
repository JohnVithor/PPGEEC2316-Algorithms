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

    for i in access_sequence {
        let (oracle_result, cost) = oracle.get(i);
        oracle_cost += cost;
        let (lru_result, cost) = lru.get(i);
        lru_cost += cost;
        if oracle_result != lru_result {
            println!("Erro: oracle {:?} != lru {:?}", oracle_result, lru_result);
            return Err(());
        }
    }
    println!("Custo total oracle: {}", oracle_cost);
    println!("Custo total lru: {}", lru_cost);

    Ok(())
}
