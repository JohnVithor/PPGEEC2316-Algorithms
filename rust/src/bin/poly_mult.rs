use std::env::args;
use std::str::FromStr;

use algorithms::algorithms::fourier_transform::multiply_polynomials;
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        println!("Usage: poly_mult <a> <b> where a and b are polynomials represented as space-separated coefficients in increasing order of degree");
        return;
    }
    let a = &args[1];
    let b = &args[2];

    let a: Vec<f64> = a
        .split(' ')
        .map(f64::from_str)
        .map(|x| x.expect("expected a valid float as coefficient"))
        .collect();
    let b: Vec<f64> = b
        .split(' ')
        .map(f64::from_str)
        .map(|x| x.expect("expected a valid float as coefficient"))
        .collect();

    let c = multiply_polynomials(&a, &b);
    println!("{:?}", c);
}
