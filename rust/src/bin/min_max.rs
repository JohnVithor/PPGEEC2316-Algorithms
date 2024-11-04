use algorithms::algorithms::minimum_maximum::{minimum_maximum, minimum_maximum_naive};
use std::{collections::VecDeque, env::args};

const SIZES: [usize; 66] = [
    10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 2000,
    3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 20000, 30000, 40000, 50000, 60000, 70000,
    80000, 90000, 100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000, 900000, 1000000,
    2000000, 3000000, 4000000, 5000000, 6000000, 7000000, 8000000, 9000000, 10000000, 20000000,
    30000000, 40000000, 50000000, 60000000, 70000000, 80000000, 90000000, 100000000, 200000000,
    250000000,
];

fn main() {
    let args: Vec<String> = args().collect();
    let path: &str = &args[1];
    let bytes = std::fs::read(path).expect("Could not read the file");
    let arr: Vec<i32> = bytes
        .chunks_exact(4)
        .map(|chunk| i32::from_le_bytes(chunk.try_into().unwrap()))
        .collect();
    println!("size,run,naive,optimized");
    let mut store = Vec::new();
    let mut naives = VecDeque::new();
    let mut optimized = VecDeque::new();

    for &size in SIZES.iter() {
        for _i in 1..101 {
            let arr = &arr[..size];

            let start = std::time::Instant::now();
            let r1 = minimum_maximum_naive(arr).unwrap();
            let naive_time = start.elapsed().as_secs_f64();
            naives.push_back(naive_time);
            store.push(r1);
        }
    }
    for &size in SIZES.iter() {
        for _i in 1..101 {
            let arr = &arr[..size];
            let start = std::time::Instant::now();
            let r2 = minimum_maximum(arr).unwrap();
            let optimized_time = start.elapsed().as_secs_f64();
            optimized.push_back(optimized_time);
            store.push(r2);
        }
    }
    for &size in SIZES.iter() {
        for i in 1..101 {
            let naive_time = naives.pop_front().unwrap();
            let optimized_time = optimized.pop_front().unwrap();
            println!("{size},{i},{:.6},{:.6}", naive_time, optimized_time);
        }
    }
    store.clear();
}
