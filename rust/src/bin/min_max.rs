use std::env::args;

fn minimum_maximum_naive<T: PartialOrd + Copy>(arr: &[T]) -> Option<(T, T)> {
    if arr.is_empty() {
        return None;
    }
    let mut min = arr[0];
    let mut max = arr[0];
    for &x in arr.iter().skip(1) {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    Some((min, max))
}

fn minimum_maximum<T: PartialOrd + Copy>(arr: &[T]) -> Option<(T, T)> {
    if arr.is_empty() {
        return None;
    }

    let mut min;
    let mut max;
    let mut i;
    if arr.len() % 2 == 0 {
        if arr[0] > arr[1] {
            min = arr[1];
            max = arr[0];
        } else {
            min = arr[0];
            max = arr[1];
        }
        i = 2;
    } else {
        min = arr[0];
        max = arr[0];
        i = 1;
    }
    while i + 1 < arr.len() {
        if arr[i] > arr[i + 1] {
            if arr[i] > max {
                max = arr[i];
            }
            if arr[i + 1] < min {
                min = arr[i + 1];
            }
        } else {
            if arr[i + 1] > max {
                max = arr[i + 1];
            }
            if arr[i] < min {
                min = arr[i];
            }
        }
        i += 2;
    }
    Some((min, max))
}

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
