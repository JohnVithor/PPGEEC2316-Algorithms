use std::{env::args, time::Instant};

use rand::{Rng, SeedableRng};

type Item = f64;
const LOWER_BOUND: Item = 0.0;
const UPPER_BOUND: Item = 1.0;

pub struct Matrix<'a> {
    pub size: usize,
    pub stride: usize,
    data: &'a [Item],
}

struct MutMatrix<'a> {
    pub size: usize,
    pub stride: usize,
    data: &'a mut [Item],
}

impl<'a> Matrix<'a> {
    fn new(size: usize, stride: usize, data: &'a [Item]) -> Matrix<'a> {
        Matrix { size, stride, data }
    }

    fn split4(&'a self) -> (Matrix<'a>, Matrix<'a>, Matrix<'a>, Matrix<'a>) {
        let mid = self.size / 2;
        let m11 = Matrix::new(mid, self.stride, self.data);
        let m12 = Matrix::new(mid, self.stride, &self.data[mid..]);
        let m21 = Matrix::new(mid, self.stride, &self.data[(mid * self.stride)..]);
        let m22 = Matrix::new(mid, self.stride, &self.data[(mid * self.stride + mid)..]);

        (m11, m12, m21, m22)
    }

    pub fn print(&self) {
        for i in 0..self.size {
            for j in 0..self.size {
                print!("{:.2} ", self[(i, j)]);
            }
            println!();
        }
    }
}

impl<'a> std::ops::Index<(usize, usize)> for Matrix<'a> {
    type Output = Item;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.stride + index.1]
    }
}

impl<'a> MutMatrix<'a> {
    fn new(size: usize, data: &'a mut [Item]) -> MutMatrix<'a> {
        MutMatrix {
            size,
            stride: size,
            data,
        }
    }

    pub fn transpose(&mut self) {
        for i in 0..self.size {
            for j in 0..i {
                self.data.swap(i * self.stride + j, j * self.stride + i);
            }
        }
    }

    fn as_matrix(&self) -> Matrix {
        Matrix {
            size: self.size,
            stride: self.stride,
            data: self.data,
        }
    }
}

impl<'a> std::ops::Index<(usize, usize)> for MutMatrix<'a> {
    type Output = Item;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.stride + index.1]
    }
}

impl<'a> std::ops::IndexMut<(usize, usize)> for MutMatrix<'a> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.stride + index.1]
    }
}

fn matrix_multiply(matrix_a: &Matrix, matrix_b: &Matrix, matrix_c: &mut MutMatrix) {
    for i in 0..matrix_a.size {
        for j in 0..matrix_a.size {
            matrix_c[(i, j)] = 0.0;
            for k in 0..matrix_a.size {
                matrix_c[(i, j)] += matrix_a[(i, k)] * matrix_b[(k, j)];
            }
        }
    }
}

fn matrix_multiply_transposed(matrix_a: &Matrix, matrix_b: &Matrix, matrix_c: &mut MutMatrix) {
    for i in 0..matrix_a.size {
        for j in 0..matrix_a.size {
            matrix_c[(i, j)] = 0.0;
            for k in 0..matrix_a.size {
                matrix_c[(i, j)] += matrix_a[(i, k)] * matrix_b[(j, k)];
            }
        }
    }
}

fn add_matrices(a: &Matrix, b: &Matrix, c: &mut MutMatrix) {
    let n = a.size;
    for i in 0..n {
        for j in 0..n {
            c[(i, j)] = a[(i, j)] + b[(i, j)];
        }
    }
}

fn subtract_matrices(a: &Matrix, b: &Matrix, c: &mut MutMatrix) {
    let n = a.size;
    for i in 0..n {
        for j in 0..n {
            c[(i, j)] = a[(i, j)] - b[(i, j)];
        }
    }
}

fn strassen(matrix_a: &Matrix, matrix_b: &Matrix, matrix_c: &mut MutMatrix, buffer: &mut [Item]) {
    let n = matrix_a.size;
    if n == 1 {
        matrix_c[(0, 0)] = matrix_a[(0, 0)] * matrix_b[(0, 0)];
        return;
    }

    let mid = n / 2;

    let (a11, a12, a21, a22) = matrix_a.split4();
    let (b11, b12, b21, b22) = matrix_b.split4();

    let (temp_buffer, buffer) = buffer.split_at_mut(mid * mid);
    let mut aux1 = MutMatrix::new(mid, temp_buffer);
    let (temp_buffer, buffer) = buffer.split_at_mut(mid * mid);
    let mut aux2 = MutMatrix::new(mid, temp_buffer);
    let (temp_buffer, buffer) = buffer.split_at_mut(mid * mid);
    let mut p1 = MutMatrix::new(mid, temp_buffer);
    let (temp_buffer, buffer) = buffer.split_at_mut(mid * mid);
    let mut p2 = MutMatrix::new(mid, temp_buffer);
    let (temp_buffer, buffer) = buffer.split_at_mut(mid * mid);
    let mut p3 = MutMatrix::new(mid, temp_buffer);
    let (temp_buffer, buffer) = buffer.split_at_mut(mid * mid);
    let mut p4 = MutMatrix::new(mid, temp_buffer);
    let (temp_buffer, buffer) = buffer.split_at_mut(mid * mid);
    let mut p5 = MutMatrix::new(mid, temp_buffer);
    let (temp_buffer, buffer) = buffer.split_at_mut(mid * mid);
    let mut p6 = MutMatrix::new(mid, temp_buffer);
    let (temp_buffer, buffer) = buffer.split_at_mut(mid * mid);
    let mut p7 = MutMatrix::new(mid, temp_buffer);

    add_matrices(&a11, &a22, &mut aux1);
    add_matrices(&b11, &b22, &mut aux2);
    strassen(&aux1.as_matrix(), &aux2.as_matrix(), &mut p1, buffer);

    add_matrices(&a21, &a22, &mut aux1);
    strassen(&aux1.as_matrix(), &b11, &mut p2, buffer);

    subtract_matrices(&b12, &b22, &mut aux2);
    strassen(&a11, &aux2.as_matrix(), &mut p3, buffer);

    subtract_matrices(&b21, &b11, &mut aux2);
    strassen(&a22, &aux2.as_matrix(), &mut p4, buffer);

    add_matrices(&a11, &a12, &mut aux1);
    strassen(&aux1.as_matrix(), &b22, &mut p5, buffer);

    subtract_matrices(&a21, &a11, &mut aux1);
    add_matrices(&b11, &b12, &mut aux2);
    strassen(&aux1.as_matrix(), &aux2.as_matrix(), &mut p6, buffer);

    subtract_matrices(&a12, &a22, &mut aux1);
    add_matrices(&b21, &b22, &mut aux2);
    strassen(&aux1.as_matrix(), &aux2.as_matrix(), &mut p7, buffer);

    for i in 0..mid {
        for j in 0..mid {
            matrix_c[(i, j)] = p1[(i, j)] + p4[(i, j)] - p5[(i, j)] + p7[(i, j)];
            matrix_c[(i, j + mid)] = p3[(i, j)] + p5[(i, j)];
            matrix_c[(i + mid, j)] = p2[(i, j)] + p4[(i, j)];
            matrix_c[(i + mid, j + mid)] = p1[(i, j)] - p2[(i, j)] + p3[(i, j)] + p6[(i, j)];
        }
    }
}

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

    for i in 0..(size * size) {
        if (matrix_c.data[i] - matrix_d.data[i]).abs() > 0.001 {
            println!(
                "Erro: c[{i}] = {} != {} = d[{i}]\n",
                matrix_c.data[i], matrix_d.data[i],
            );
            return Err(());
        }
    }

    let now = Instant::now();
    matrix_b.transpose();
    matrix_multiply_transposed(&matrix_a, &matrix_b.as_matrix(), &mut matrix_d);
    matrix_b.transpose();
    println!("{:.6?}", now.elapsed().as_secs_f64());
    Ok(())
}
