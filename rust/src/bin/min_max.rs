use std::env::args;

use algorithms::minimum_maximum::{minimum_maximum, minimum_maximum_naive};

fn main() {
    type T = i32;
    let n: T = args()
        .nth(1)
        .expect("Please provide the number of elements")
        .parse()
        .unwrap();
    let arr = (0..n).collect::<Vec<T>>();
    let start = std::time::Instant::now();
    match minimum_maximum(&arr) {
        Some((min, max)) => println!("Minimum: {}, Maximum: {}", min, max),
        None => println!("Array is empty"),
    }
    println!("Time taken: {:?}", start.elapsed());
    let start = std::time::Instant::now();
    match minimum_maximum_naive(&arr) {
        Some((min, max)) => println!("Minimum: {}, Maximum: {}", min, max),
        None => println!("Array is empty"),
    }
    println!("Time taken: {:?}", start.elapsed());
}
