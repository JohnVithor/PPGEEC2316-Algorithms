use std::env::args;

use algorithms::algorithms::fourier_transform::{compute_dft, Complex};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Usage: dft <n>");
        return;
    }
    let n: usize = args[1].parse().expect("Usage: dft <n>");
    let input: Vec<Complex> = (0..n)
        .map(|_| Complex(fastrand::f64(), fastrand::f64()))
        .collect();
    let now = std::time::Instant::now();
    let _ = compute_dft(&input);
    let elapsed = now.elapsed();
    println!("{:?}", elapsed);
}
