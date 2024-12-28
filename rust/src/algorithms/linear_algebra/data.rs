use std::fmt::Debug;

const EPSILON: f64 = 1e-10;

#[derive(Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize>
where
    [(); R * C]:,
{
    pub data: [f64; R * C],
}

impl<const R: usize, const C: usize> Matrix<R, C>
where
    [(); R * C]:,
{
    pub fn new(default: f64) -> Self {
        Self {
            data: [default; R * C],
        }
    }

    pub fn identity() -> Self {
        let mut data = [0.0; R * C];
        for i in 0..R {
            data[i * C + i] = 1.0;
        }
        Self { data }
    }

    pub fn swap(&mut self, i: (usize, usize), j: (usize, usize)) {
        self.data.swap(i.0 * C + i.1, j.0 * C + j.1);
    }
}

impl<const S: usize> Matrix<S, S>
where
    [(); S * S]:,
{
    pub fn lu(self) -> (Matrix<S, S>, Matrix<S, S>)
    where
        [(); S * S]:,
    {
        let mut l = Matrix::new(0.0);
        let mut u = self;
        for k in 0..S {
            l[(k, k)] = 1.0;
            for i in (k + 1)..S {
                l[(i, k)] = u[(i, k)];
                u[(i, k)] = 0.0;
            }
        }
        (l, u)
    }
}

impl<const R: usize, const C: usize> std::ops::Index<(usize, usize)> for Matrix<R, C>
where
    [(); R * C]:,
{
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * C + index.1]
    }
}

impl<const R: usize, const C: usize> std::ops::IndexMut<(usize, usize)> for Matrix<R, C>
where
    [(); R * C]:,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * C + index.1]
    }
}

impl<const R: usize, const C: usize> From<[[f64; C]; R]> for Matrix<R, C>
where
    [(); R * C]:,
{
    fn from(input: [[f64; C]; R]) -> Self {
        let mut data = [0.0; R * C];
        for i in 0..R {
            for j in 0..C {
                data[i * C + j] = input[i][j];
            }
        }
        Self { data }
    }
}

impl<const R: usize, const C: usize> PartialEq for Matrix<R, C>
where
    [(); R * C]:,
{
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter()
            .zip(other.data.iter())
            .all(|(a, b)| (a - b).abs() < EPSILON)
    }
}

impl<const R: usize, const C: usize> Debug for Matrix<R, C>
where
    [(); R * C]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data = [[0.0; C]; R];
        for i in 0..R {
            for j in 0..C {
                data[i][j] = self[(i, j)];
            }
        }
        f.debug_list().entries(data).finish()
    }
}

impl<const R: usize, const C: usize> std::ops::Mul<Vector<C>> for Matrix<R, C>
where
    [(); R * C]:,
{
    type Output = Vector<R>;

    fn mul(self, rhs: Vector<C>) -> Self::Output {
        let mut result = Vector::new(0.0);
        for i in 0..R {
            let mut sum = 0.0;
            for j in 0..C {
                sum += self[(i, j)] * rhs[j];
            }
            result[i] = sum;
        }
        result
    }
}

impl<const R: usize, const C: usize> std::ops::Mul<&Vector<C>> for &Matrix<R, C>
where
    [(); R * C]:,
{
    type Output = Vector<R>;

    fn mul(self, rhs: &Vector<C>) -> Self::Output {
        let mut result = Vector::new(0.0);
        for i in 0..R {
            let mut sum = 0.0;
            for j in 0..C {
                sum += self[(i, j)] * rhs[j];
            }
            result[i] = sum;
        }
        result
    }
}

// Vector

pub struct Vector<const S: usize> {
    pub data: [f64; S],
}

impl<const S: usize> Vector<S> {
    pub fn new(default: f64) -> Self {
        Self { data: [default; S] }
    }
}

impl<const S: usize> std::ops::Index<usize> for Vector<S> {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const S: usize> std::ops::IndexMut<usize> for Vector<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const S: usize> From<[f64; S]> for Vector<S> {
    fn from(data: [f64; S]) -> Self {
        Self { data }
    }
}

impl<const S: usize> PartialEq for Vector<S>
where
    [f64; S]:,
{
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter()
            .zip(other.data.iter())
            .all(|(a, b)| (a - b).abs() < EPSILON)
    }
}

impl<const S: usize> Debug for Vector<S>
where
    [f64; S]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data).finish()
    }
}
