use std::env::args;

use rand::{seq::SliceRandom, SeedableRng};

fn merge(left: &[i32], right: &[i32], ret: &mut [i32]) {
    let mut l = 0;
    let mut r = 0;
    let mut i = 0;

    while l < left.len() && r < right.len() {
        ret[i] = if left[l] < right[r] {
            l += 1;
            left[l - 1]
        } else {
            r += 1;
            right[r - 1]
        };
        i += 1;
    }

    if l < left.len() {
        for (a, l) in ret[i..].iter_mut().zip(left[l..].iter()) {
            *a = *l;
        }
    } else if r < right.len() {
        for (a, r) in ret[i..].iter_mut().zip(left[r..].iter()) {
            *a = *r;
        }
    }
}

fn merge_sort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let aux = arr.to_vec();
    merge(&aux[..mid], &aux[mid..], arr);
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
    merge_sort(&mut arr);
    let elapsed_worse = now.elapsed();

    let now = std::time::Instant::now();
    merge_sort(&mut arr);
    let elapsed_best = now.elapsed();

    arr.shuffle(&mut rand::rngs::SmallRng::seed_from_u64(seed));

    let now = std::time::Instant::now();
    merge_sort(&mut arr);
    let elapsed_avg = now.elapsed();

    println!(
        "{},{},{}",
        elapsed_worse.as_secs_f64(),
        elapsed_best.as_secs_f64(),
        elapsed_avg.as_secs_f64(),
    );
}
