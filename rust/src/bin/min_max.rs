use algorithms::algorithms::minimum_maximum::{minimum_maximum, minimum_maximum_naive};
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let n: usize = args[1].parse().unwrap();
    let seed: u64 = args[2].parse().unwrap();
    fastrand::seed(seed);
    let start = std::time::Instant::now();
    let arr: Vec<i32> = (0..n).map(|_| fastrand::i32(i32::MIN..i32::MAX)).collect();
    let time_to_generate = start.elapsed().as_secs_f64();
    let start = std::time::Instant::now();
    let r1 = minimum_maximum_naive(&arr).unwrap();
    let naive_time = start.elapsed().as_secs_f64();
    let start = std::time::Instant::now();
    let r2 = minimum_maximum(&arr).unwrap();
    let optimized_time = start.elapsed().as_secs_f64();
    if r1.0 != r2.0 || r1.1 != r2.1 {
        println!(
            "Valores diferentes:\nmin: {} e {}\nmax: {} e {}\n ",
            r1.0, r2.0, r1.1, r2.1,
        );
    }
    println!(
        "{:.6},{:.6},{:.6}",
        time_to_generate, naive_time, optimized_time
    );
}
