use algorithms::algorithms::minimum_maximum::{minimum_maximum, minimum_maximum_naive};
use std::env::args;

const SIZE_MAX: usize = 250000000;
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
    let seed: u64 = args[1].parse().unwrap();
    fastrand::seed(seed);
    let mut arr: Vec<usize> = (0..SIZE_MAX).collect();
    for i in 0..SIZE_MAX {
        arr.swap(i, fastrand::usize(..SIZE_MAX));
    }
    println!("size,run,naive,optimized");
    for &size in SIZES.iter() {
        for i in 1..101 {
            let arr = &arr[..size];

            let start = std::time::Instant::now();
            let r1 = minimum_maximum_naive(arr).unwrap();
            let naive_time = start.elapsed().as_secs_f64();
            let start = std::time::Instant::now();
            let r2 = minimum_maximum(arr).unwrap();
            let optimized_time = start.elapsed().as_secs_f64();
            if r1.0 != r2.0 || r1.1 != r2.1 {
                println!(
                    "Valores diferentes:\nmin: {} e {}\nmax: {} e {}\n ",
                    r1.0, r2.0, r1.1, r2.1,
                );
            }
            println!("{size},{i},{:.6},{:.6}", naive_time, optimized_time);
        }
    }
}
