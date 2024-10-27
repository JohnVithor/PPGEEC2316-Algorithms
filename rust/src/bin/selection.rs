use algorithms::algorithms::selection::median_of_medians::select_kth;
use algorithms::algorithms::selection::randomized_selection::randomized_select_kth;

use std::env;
use std::fmt::Display;
use std::time::Instant;

const SIZE_MAX: usize = 250_000_000;

fn measure_print<T: Ord + Display + Copy>(arr: &mut [T], i: usize, name: &str) {
    for j in 1..11 {
        let start = Instant::now();
        let &r1 = randomized_select_kth(arr, i);
        let time_spent_rand = start.elapsed().as_secs_f64();

        let start = Instant::now();
        let &r2 = select_kth(arr, i);
        let time_spent_med = start.elapsed().as_secs_f64();

        if r1 != r2 {
            println!("Valores diferentes: {} e {}", r1, r2);
        }
        println!(
            "{},{},{},{},{}",
            arr.len(),
            name,
            j,
            time_spent_rand,
            time_spent_med
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Uso: {} <seed> (seed >=0)", args[0]);
        std::process::exit(1);
    }

    let seed: u64 = match args[1].parse::<i64>() {
        Ok(n) if n >= 0 => n as u64,
        _ => {
            println!("Uso: {} <seed> (seed >=0)", args[0]);
            std::process::exit(1);
        }
    };

    fastrand::seed(seed);

    let mut arr: Vec<i32> = (0..SIZE_MAX as i32).collect();

    for i in 0..SIZE_MAX {
        let j = fastrand::usize(0..SIZE_MAX);
        arr.swap(i, j);
    }

    let sizes: [usize; 66] = [
        10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000,
        2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 20000, 30000, 40000, 50000, 60000,
        70000, 80000, 90000, 100000, 200000, 300000, 400000, 500000, 600000, 700000, 800000,
        900000, 1000000, 2000000, 3000000, 4000000, 5000000, 6000000, 7000000, 8000000, 9000000,
        10000000, 20000000, 30000000, 40000000, 50000000, 60000000, 70000000, 80000000, 90000000,
        100000000, 200000000, 250000000,
    ];

    println!("size,i,run,randomized,median_of_medians");

    for &size in sizes.iter() {
        measure_print(&mut arr[..size], 1, "min");
        measure_print(&mut arr[..size], size / 10, "1/10");
        measure_print(&mut arr[..size], size / 4, "1/4");
        measure_print(&mut arr[..size], size / 3, "1/3");
        measure_print(&mut arr[..size], size / 2, "median");
        measure_print(&mut arr[..size], 2 * size / 3, "2/3");
        measure_print(&mut arr[..size], 3 * size / 4, "3/4");
        measure_print(&mut arr[..size], 9 * size / 10, "9/10");
        measure_print(&mut arr[..size], size, "max");
    }
}
