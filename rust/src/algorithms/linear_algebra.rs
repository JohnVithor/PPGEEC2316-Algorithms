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
    L: &Matrix<S, S>,
    U: &Matrix<S, S>,
    P: &Matrix<S, S>,
    b: &Vector<S>,
) -> Vector<S>
where
    [(); S * S]:,
{
    backward_substitution(U, &forward_substitution(L, &(P * b)))
}

pub fn lu_decomposition<const S: usize>(mut matrix: Matrix<S, S>) -> (Matrix<S, S>, Matrix<S, S>)
where
    [(); S * S]:,
{
    let mut L = Matrix::new(0.0);
    let mut U = Matrix::new(0.0);
    for k in 0..S {
        L[(k, k)] = 1.0;
        U[(k, k)] = matrix[(k, k)];
        for i in k..S {
            L[(i, k)] = matrix[(i, k)] / matrix[(k, k)];
            U[(k, i)] = matrix[(k, i)];
        }
        for i in k..S {
            for j in k..S {
                matrix[(i, j)] -= L[(i, k)] * U[(k, j)];
            }
        }
    }
    (L, U)
}

pub fn lup_decomposition<const S: usize>(
    mut matrix: Matrix<S, S>,
) -> (Matrix<S, S>, Matrix<S, S>, Matrix<S, S>)
where
    [(); S * S]:,
{
    let mut p = [0; S];
    for i in 0..S {
        p[i] = i;
    }
    todo!("Implement LUP decomposition")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_substitution() {
        // let A = Matrix {
        //     data: [1.0, 2.0, 0.0, 3.0, 4.0, 4.0, 5.0, 6.0, 3.0],
        // };
        let b = Vector::from([3.0, 7.0, 8.0]);
        let L = Matrix::from([[1.0, 0.0, 0.0], [0.2, 1.0, 0.0], [0.6, 0.5, 1.0]]);
        let P = Matrix::from([[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
        let Pb = P * b;
        let result = forward_substitution(&L, &Pb);
        assert_eq!(result, Vector::from([8.0, 1.4, 1.5]));
    }

    #[test]
    fn test_backward_substitution() {
        let U = Matrix::from([[5.0, 6.0, 3.0], [0.0, 0.8, -0.6], [0.0, 0.0, 2.5]]);
        let y = Vector::from([8.0, 1.4, 1.5]);
        let result = backward_substitution(&U, &y);
        assert_eq!(result, Vector::from([-1.4, 2.2, 0.6]));
    }

    #[test]
    fn test_lup_solve() {
        let L = Matrix::from([[1.0, 0.0, 0.0], [0.2, 1.0, 0.0], [0.6, 0.5, 1.0]]);
        let U = Matrix::from([[5.0, 6.0, 3.0], [0.0, 0.8, -0.6], [0.0, 0.0, 2.5]]);
        let P = Matrix::from([[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]);
        let b = Vector::from([3.0, 7.0, 8.0]);
        let result = lup_solve(&L, &U, &P, &b);
        assert_eq!(result, Vector::from([-1.4, 2.2, 0.6]));
    }

    #[test]
    fn test_lu_decomposition() {
        let A = Matrix::from([
            [2.0, 3.0, 1.0, 5.0],
            [6.0, 13.0, 5.0, 19.0],
            [2.0, 19.0, 10.0, 23.0],
            [4.0, 10.0, 11.0, 31.0],
        ]);
        let (L, U) = lu_decomposition(A);
        assert_eq!(
            L,
            Matrix::from([
                [1.0, 0.0, 0.0, 0.0],
                [3.0, 1.0, 0.0, 0.0],
                [1.0, 4.0, 1.0, 0.0],
                [2.0, 1.0, 7.0, 1.0],
            ])
        );
        assert_eq!(
            U,
            Matrix::from([
                [2.0, 3.0, 1.0, 5.0],
                [0.0, 4.0, 2.0, 4.0],
                [0.0, 0.0, 1.0, 2.0],
                [0.0, 0.0, 0.0, 3.0],
            ])
        );
    }
}
