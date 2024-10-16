use std::ops::{Add, Mul, Sub};

pub struct Matrix<'a, Item> {
    pub size: usize,
    pub stride: usize,
    data: &'a [Item],
}

pub struct MutMatrix<'a, Item> {
    pub size: usize,
    pub stride: usize,
    data: &'a mut [Item],
}

impl<'a, Item> Matrix<'a, Item> {
    pub fn new(size: usize, stride: usize, data: &'a [Item]) -> Matrix<'a, Item> {
        Matrix { size, stride, data }
    }

    fn split4(
        &'a self,
    ) -> (
        Matrix<'a, Item>,
        Matrix<'a, Item>,
        Matrix<'a, Item>,
        Matrix<'a, Item>,
    ) {
        let mid = self.size / 2;
        let m11 = Matrix::new(mid, self.stride, self.data);
        let m12 = Matrix::new(mid, self.stride, &self.data[mid..]);
        let m21 = Matrix::new(mid, self.stride, &self.data[(mid * self.stride)..]);
        let m22 = Matrix::new(mid, self.stride, &self.data[(mid * self.stride + mid)..]);

        (m11, m12, m21, m22)
    }
}

impl<'a, Item> std::ops::Index<(usize, usize)> for Matrix<'a, Item> {
    type Output = Item;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.stride + index.1]
    }
}

impl<'a, Item> MutMatrix<'a, Item> {
    pub fn new(size: usize, data: &'a mut [Item]) -> MutMatrix<'a, Item> {
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

    pub fn as_matrix(&self) -> Matrix<Item> {
        Matrix {
            size: self.size,
            stride: self.stride,
            data: self.data,
        }
    }
}

impl<'a, Item> std::ops::Index<(usize, usize)> for MutMatrix<'a, Item> {
    type Output = Item;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.stride + index.1]
    }
}

impl<'a, Item> std::ops::IndexMut<(usize, usize)> for MutMatrix<'a, Item> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.stride + index.1]
    }
}

pub fn matrix_multiply<Item: Default + Copy + Mul<Output = Item> + Add<Output = Item>>(
    matrix_a: &Matrix<Item>,
    matrix_b: &Matrix<Item>,
    matrix_c: &mut MutMatrix<Item>,
) {
    for i in 0..matrix_a.size {
        for j in 0..matrix_a.size {
            matrix_c[(i, j)] = Item::default();
            for k in 0..matrix_a.size {
                matrix_c[(i, j)] = matrix_c[(i, j)] + matrix_a[(i, k)] * matrix_b[(k, j)];
            }
        }
    }
}

pub fn matrix_multiply_transposed<
    Item: Default + Copy + Mul<Output = Item> + Add<Output = Item>,
>(
    matrix_a: &Matrix<Item>,
    matrix_b: &Matrix<Item>,
    matrix_c: &mut MutMatrix<Item>,
) {
    for i in 0..matrix_a.size {
        for j in 0..matrix_a.size {
            matrix_c[(i, j)] = Item::default();
            for k in 0..matrix_a.size {
                matrix_c[(i, j)] = matrix_c[(i, j)] + matrix_a[(i, k)] * matrix_b[(j, k)];
            }
        }
    }
}

pub fn add_matrices<Item: Copy + Add<Output = Item>>(
    a: &Matrix<Item>,
    b: &Matrix<Item>,
    c: &mut MutMatrix<Item>,
) {
    let n = a.size;
    for i in 0..n {
        for j in 0..n {
            c[(i, j)] = a[(i, j)] + b[(i, j)];
        }
    }
}

pub fn subtract_matrices<Item: Copy + Sub<Output = Item>>(
    a: &Matrix<Item>,
    b: &Matrix<Item>,
    c: &mut MutMatrix<Item>,
) {
    let n = a.size;
    for i in 0..n {
        for j in 0..n {
            c[(i, j)] = a[(i, j)] - b[(i, j)];
        }
    }
}

pub fn strassen<
    Item: Default + Copy + Mul<Output = Item> + Add<Output = Item> + Sub<Output = Item>,
>(
    matrix_a: &Matrix<Item>,
    matrix_b: &Matrix<Item>,
    matrix_c: &mut MutMatrix<Item>,
    buffer: &mut [Item],
) {
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
