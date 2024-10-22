use std::env::args;

use algorithms::algorithms::sort;
use rand::{distributions::Uniform, Rng, SeedableRng};

fn main() -> Result<(), ()> {
    let args: Vec<String> = args().collect();
    if args.len() != 4 {
        println!(
            r#"
            Uso: {} <n> <seed> <algoritmo> (n = 2^x, para algum x >= 1 e seed >=0])
            Algoritmo:
            0 - insertion sort
            1 - merge sort (recursivo)
            2 - merge sort (iterativo)
            3 - quick sort (recursivo)
            4 - quick sort (iterativo)
            5 - randomized quick sort (recursivo)
            6 - randomized quick sort (iterativo)
            7 - count sort
            8 - radix sort
            "#,
            args[0],
        );
        return Err(());
    }
    let size: usize = args[1].parse().unwrap();
    let seed: i64 = args[2].parse().unwrap();
    let alg: i64 = args[3].parse().unwrap();

    if size < 2 || seed < 0 || !(0..=8).contains(&alg) {
        println!(
            "Uso: {} <n> <seed> (n = 2^x, para algum x >= 1 e seed >=0])",
            args[0],
        );
        return Err(());
    }

    let fn_alg = match alg {
        0 => sort::insertion_sort::insertion_sort,
        1 => sort::mergesort::mergesort,
        2 => sort::mergesort_iterative::mergesort,
        3 => sort::quicksort::quicksort,
        4 => sort::quicksort_iterative::quicksort,
        5 => sort::randomized_quicksort::quicksort,
        6 => sort::randomized_quicksort_iterative::quicksort,
        7 => sort::count_sort::count_sort,
        8 => sort::radix_sort::radix_sort,
        _ => unreachable!(),
    };

    let name_alg = match alg {
        0 => "insertion sort",
        1 => "merge sort (recursivo)",
        2 => "merge sort (iterativo)",
        3 => "quick sort (recursivo)",
        4 => "quick sort (iterativo)",
        5 => "randomized quick sort (recursivo)",
        6 => "randomized quick sort (iterativo)",
        7 => "count sort",
        8 => "radix sort",
        _ => unreachable!(),
    };
    let rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let mut arr = rng
        .sample_iter(Uniform::new(0, i32::MAX as usize))
        .take(size)
        .collect::<Vec<usize>>();
    // println!("{:?}", arr);
    let now = std::time::Instant::now();
    (fn_alg)(&mut arr);
    print!("{},{:.6},", name_alg, now.elapsed().as_secs_f64());
    // println!("{:?}", arr);
    for i in 0..arr.len() - 1 {
        assert!(
            arr[i] <= arr[i + 1],
            "{} {} > {} {}",
            arr[i],
            i,
            arr[i + 1],
            i + 1
        );
    }
    let now = std::time::Instant::now();
    (fn_alg)(&mut arr);
    print!("{:.6},", now.elapsed().as_secs_f64());
    arr.reverse();
    let now = std::time::Instant::now();
    (fn_alg)(&mut arr);
    println!("{:.6}", now.elapsed().as_secs_f64());

    Ok(())
}
