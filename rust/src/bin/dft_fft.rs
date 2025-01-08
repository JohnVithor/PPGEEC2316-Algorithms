use std::env::args;

use algorithms::algorithms::fourier_transform::{compute_dft, compute_fft, Complex};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        println!("Usage: dft <n> <seed>");
        return;
    }
    let n: usize = args[1].parse().expect("Usage: dft <n> <seed>");
    let seed: u64 = args[2].parse().expect("Usage: dft <n> <seed>");
    fastrand::seed(seed);
    let input: Vec<Complex> = (0..n)
        .map(|_| Complex(fastrand::f64(), fastrand::f64()))
        .collect();
    let now = std::time::Instant::now();
    let _ = compute_dft(&input);
    let elapsed_dft = now.elapsed();

    let now = std::time::Instant::now();
    let _ = compute_fft(&input);
    let elapsed_fft = now.elapsed();

    println!("{:?},{:?}", elapsed_dft, elapsed_fft);
}
