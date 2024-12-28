mod data;

use data::{Matrix, Vector};

pub fn forward_substitution<const R: usize, const C: usize>(
    matrix: &Matrix<R, C>,
    b: &Vector<C>,
) -> Vector<R>
where
    [(); R * C]:,
{
    let mut result = Vector::new(0.0);
    for i in 0..R {
        let mut sum = 0.0;
        for j in 0..i {
            sum += matrix[(i, j)] * result[j];
        }
        result[i] = b[i] - sum;
    }
    result
}

pub fn backward_substitution<const R: usize, const C: usize>(
    matrix: &Matrix<R, C>,
    b: &Vector<C>,
) -> Vector<R>
where
    [(); R * C]:,
{
    let mut result = Vector::new(0.0);
    for i in (0..R).rev() {
        let mut sum = 0.0;
        for j in (i + 1)..R {
            sum += matrix[(i, j)] * result[j];
        }
        result[i] = (b[i] - sum) / matrix[(i, i)];
    }
    result
}

pub fn lup_solve<const S: usize>(
    l: &Matrix<S, S>,
    u: &Matrix<S, S>,
    p: &Matrix<S, S>,
    b: &Vector<S>,
) -> Vector<S>
where
    [(); S * S]:,
{
    backward_substitution(u, &forward_substitution(l, &(p * b)))
}

pub fn lu_decomposition<const S: usize>(mut matrix: Matrix<S, S>) -> (Matrix<S, S>, Matrix<S, S>)
where
    [(); S * S]:,
{
    let mut l = Matrix::new(0.0);
    let mut u = Matrix::new(0.0);
    for k in 0..S {
        l[(k, k)] = 1.0;
        u[(k, k)] = matrix[(k, k)];
        for i in k..S {
            l[(i, k)] = matrix[(i, k)] / matrix[(k, k)];
            u[(k, i)] = matrix[(k, i)];
        }
        for i in k..S {
            for j in k..S {
                matrix[(i, j)] -= l[(i, k)] * u[(k, j)];
            }
        }
    }
    (l, u)
}

pub fn lup_decomposition<const S: usize>(mut matrix: Matrix<S, S>) -> (Matrix<S, S>, Matrix<S, S>)
where
    [(); S * S]:,
{
    let mut pi = [0; S];
    #[allow(clippy::needless_range_loop)]
    for i in 0..S {
        pi[i] = i;
    }
    for k in 0..S {
        let mut p = 0.0;
        let mut k_p = 0;
        for i in k..S {
            if matrix[(i, k)].abs() > p {
                p = matrix[(i, k)].abs();
                k_p = i;
            }
        }
        if p == 0.0 {
            panic!("Matrix is singular");
        }
        pi.swap(k_p, k);
        for i in 0..S {
            matrix.swap((k, i), (k_p, i));
        }
        for i in (k + 1)..S {
            matrix[(i, k)] /= matrix[(k, k)];
            for j in (k + 1)..S {
                matrix[(i, j)] -= matrix[(i, k)] * matrix[(k, j)];
            }
        }
    }

    let mut p = Matrix::new(0.0);
    for i in 0..S {
        p[(i, pi[i])] = 1.0;
    }

    (matrix, p)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_forward_substitution() {
        let b = Vector::from([3.0, 7.0, 8.0]);
        let l = Matrix::from([[1.0, 0.0, 0.0], [0.2, 1.0, 0.0], [0.6, 0.5, 1.0]]);
        let p = Matrix::from([[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
        let pb = p * b;
        let result = forward_substitution(&l, &pb);
        assert_eq!(result, Vector::from([8.0, 1.4, 1.5]));
    }

    #[test]
    fn test_backward_substitution() {
        let u = Matrix::from([[5.0, 6.0, 3.0], [0.0, 0.8, -0.6], [0.0, 0.0, 2.5]]);
        let y = Vector::from([8.0, 1.4, 1.5]);
        let result = backward_substitution(&u, &y);
        assert_eq!(result, Vector::from([-1.4, 2.2, 0.6]));
    }

    #[test]
    fn test_lup_solve() {
        let l = Matrix::from([[1.0, 0.0, 0.0], [0.2, 1.0, 0.0], [0.6, 0.5, 1.0]]);
        let u = Matrix::from([[5.0, 6.0, 3.0], [0.0, 0.8, -0.6], [0.0, 0.0, 2.5]]);
        let p = Matrix::from([[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
        let b = Vector::from([3.0, 7.0, 8.0]);
        let result = lup_solve(&l, &u, &p, &b);
        assert_eq!(result, Vector::from([-1.4, 2.2, 0.6]));
    }

    #[test]
    fn test_lu_decomposition() {
        let a = Matrix::from([
            [2.0, 3.0, 1.0, 5.0],
            [6.0, 13.0, 5.0, 19.0],
            [2.0, 19.0, 10.0, 23.0],
            [4.0, 10.0, 11.0, 31.0],
        ]);
        let (l, u) = lu_decomposition(a);
        assert_eq!(
            l,
            Matrix::from([
                [1.0, 0.0, 0.0, 0.0],
                [3.0, 1.0, 0.0, 0.0],
                [1.0, 4.0, 1.0, 0.0],
                [2.0, 1.0, 7.0, 1.0],
            ])
        );
        assert_eq!(
            u,
            Matrix::from([
                [2.0, 3.0, 1.0, 5.0],
                [0.0, 4.0, 2.0, 4.0],
                [0.0, 0.0, 1.0, 2.0],
                [0.0, 0.0, 0.0, 3.0],
            ])
        );
    }

    #[test]
    fn test_lup_decomposition() {
        let a = Matrix::from([
            [2.0, 0.0, 2.0, 0.6],
            [3.0, 3.0, 4.0, -2.0],
            [5.0, 5.0, 4.0, 2.0],
            [-1.0, -2.0, 3.4, -1.0],
        ]);
        let (lu, p) = lup_decomposition(a);
        let (l, u) = lu.lu();
        assert_eq!(
            l,
            Matrix::from([
                [1.0, 0.0, 0.0, 0.0],
                [0.4, 1.0, 0.0, 0.0],
                [-0.2, 0.5, 1.0, 0.0],
                [0.6, 0.0, 0.4, 1.0],
            ])
        );
        assert_eq!(
            u,
            Matrix::from([
                [5.0, 5.0, 4.0, 2.0],
                [0.0, -2.0, 0.4, -0.2],
                [0.0, 0.0, 4.0, -0.5],
                [0.0, 0.0, 0.0, -3.0],
            ])
        );
        assert_eq!(
            p,
            Matrix::from([
                [0.0, 0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 0.0],
            ])
        );
    }

    #[test]
    fn test_inverse() {
        let a = Matrix::from([
            [-3.0, -1.0, 2.0, -3.0],
            [-3.0, 1.0, 2.0, -2.0],
            [-2.0, 3.0, 0.0, 1.0],
            [1.0, -2.0, -3.0, 1.0],
        ]);
        let (lu, p) = lup_decomposition(a);
        let (l, u) = lu.lu();
        let mut inv: Matrix<4, 4> = Matrix::new(0.0);
        for i in 0..4 {
            let mut b = Vector::new(0.0);
            b[i] = 1.0;
            let y = forward_substitution(&l, &(&p * &b));
            let x = backward_substitution(&u, &y);
            for j in 0..4 {
                inv[(j, i)] = x[j];
            }
        }
        // Check that A * A^-1 = I
        let mut r = Matrix::new(0.0);
        for i in 0..4 {
            for j in 0..4 {
                r[(i, j)] = 0.0;
                for k in 0..4 {
                    r[(i, j)] += inv[(i, k)] * a[(k, j)];
                }
            }
        }
        let i: Matrix<4, 4> = Matrix::identity();
        assert_eq!(r, i);
    }
}
