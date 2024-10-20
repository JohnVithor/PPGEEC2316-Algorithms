use std::{env::args, time::Instant};

use algorithms::data_structures::matrix::{
    matrix_multiply, matrix_multiply_transposed, strassen, Matrix, MutMatrix,
};
use rand::{Rng, SeedableRng};

type Item = f64;
const LOWER_BOUND: Item = 0.0;
const UPPER_BOUND: Item = 1.0;

fn main() -> Result<(), ()> {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        println!(
            "Uso: {} <n> <seed> (n = 2^x, para algum x >= 1 e seed >=0])",
            args[0],
        );
        return Err(());
    }
    let size: usize = args[1].parse().unwrap();
    let seed: i64 = args[2].parse().unwrap();

    if size < 2 || (size & (size - 1)) != 0 || seed < 0 {
        println!(
            "Uso: {} <n> <seed> (n = 2^x, para algum x >= 1 e seed >=0])",
            args[0],
        );
        return Err(());
    }

    let a = rand::rngs::StdRng::seed_from_u64(0)
        .sample_iter(rand::distributions::Uniform::new(LOWER_BOUND, UPPER_BOUND))
        .take(size * size)
        .collect::<Vec<Item>>();
    let mut b = rand::rngs::StdRng::seed_from_u64(1)
        .sample_iter(rand::distributions::Uniform::new(LOWER_BOUND, UPPER_BOUND))
        .take(size * size)
        .collect::<Vec<Item>>();

    // let a: Vec<Item> = (1..=(size * size)).map(|x| x as Item).collect();
    // let mut b: Vec<Item> = (1..=(size * size)).map(|x| x as Item).collect();

    let mut c = vec![0.0; size * size];
    let mut d = vec![0.0; size * size];
    let mut buffer = vec![0.0; 4 * size * size];

    let matrix_a = Matrix::new(size, size, &a);
    let mut matrix_b = MutMatrix::new(size, &mut b);
    let mut matrix_c = MutMatrix::new(size, &mut c);
    let mut matrix_d = MutMatrix::new(size, &mut d);

    let now = Instant::now();
    strassen(&matrix_a, &matrix_b.as_matrix(), &mut matrix_c, &mut buffer);
    print!("{:.6?},", now.elapsed().as_secs_f64());

    let now = Instant::now();
    matrix_multiply(&matrix_a, &matrix_b.as_matrix(), &mut matrix_d);
    print!("{:.6?},", now.elapsed().as_secs_f64());

    for i in 0..size {
        for j in 0..size {
            if (matrix_c[(i, j)] - matrix_d[(i, j)]).abs() > 0.001 {
                println!(
                    "Erro: c[{i}][{j}] = {} != {} = d[{i}][{j}]\n",
                    matrix_c[(i, j)],
                    matrix_d[(i, j)],
                );
                return Err(());
            }
        }
    }

    let now = Instant::now();
    matrix_b.transpose();
    matrix_multiply_transposed(&matrix_a, &matrix_b.as_matrix(), &mut matrix_d);
    matrix_b.transpose();
    println!("{:.6?}", now.elapsed().as_secs_f64());
    Ok(())
}
