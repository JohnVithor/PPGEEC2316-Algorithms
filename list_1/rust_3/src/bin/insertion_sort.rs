use std::env::args;

use rand::{seq::SliceRandom, SeedableRng};

fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn main() {
    let prog_name = args().next().unwrap();
    let n = args()
        .nth(1)
        .unwrap_or_else(|| panic!("Uso: {prog_name} <n> <seed> (n > 1 e seed >=0)"))
        .parse::<usize>()
        .expect("<n> deve ser um número natural maior que 1");
    if n < 2 {
        panic!("<n> deve ser um número natural maior que 1");
    }
    let seed = args()
        .nth(2)
        .unwrap_or_else(|| panic!("Uso: {prog_name} <n> <seed> (n > 1 e seed >=0)"))
        .parse::<u64>()
        .expect("<seed> deve ser um número natural");
    let mut arr = (0..n as i32).rev().collect::<Vec<_>>();

    let now = std::time::Instant::now();
    insertion_sort(&mut arr);
    let elapsed_worse = now.elapsed();

    let now = std::time::Instant::now();
    insertion_sort(&mut arr);
    let elapsed_best = now.elapsed();

    arr.shuffle(&mut rand::rngs::SmallRng::seed_from_u64(seed));

    let now = std::time::Instant::now();
    insertion_sort(&mut arr);
    let elapsed_avg = now.elapsed();

    println!(
        "{},{},{}",
        elapsed_worse.as_secs_f64(),
        elapsed_best.as_secs_f64(),
        elapsed_avg.as_secs_f64(),
    );
}
