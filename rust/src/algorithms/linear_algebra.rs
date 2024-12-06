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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_substitution() {
        // let A = Matrix {
        //     data: [1.0, 2.0, 0.0, 3.0, 4.0, 4.0, 5.0, 6.0, 3.0],
        // };
        let b = Vector::from([3.0, 7.0, 8.0]);
        let L: Matrix<3, 3> = Matrix {
            data: [1.0, 0.0, 0.0, 0.2, 1.0, 0.0, 0.6, 0.5, 1.0],
        };

        let P = Matrix {
            data: [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        };
        let Pb = P * b;
        let result = forward_substitution(&L, &Pb);
        assert_eq!(result, Vector::from([8.0, 1.4, 1.5]));
    }

    #[test]
    fn test_backward_substitution() {
        let U = Matrix {
            data: [5.0, 6.0, 3.0, 0.0, 0.8, -0.6, 0.0, 0.0, 2.5],
        };
        let y = Vector::from([8.0, 1.4, 1.5]);
        let result = backward_substitution(&U, &y);
        assert_eq!(result, Vector::from([-1.4, 2.2, 0.6]));
    }
}
