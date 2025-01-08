#[derive(Debug, Clone, Copy)]
pub struct Complex(pub f64, pub f64);

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 1e-9 && (self.1 - other.1).abs() < 1e-9
    }
}
impl Eq for Complex {}

pub fn compute_dft(input: &[Complex]) -> Vec<Complex> {
    let n: usize = input.len();
    (0..n)
        .map(|k| {
            let mut sumreal = 0.0;
            let mut sumimag = 0.0;
            for (t, Complex(r, i)) in input.iter().enumerate() {
                let angle = 2.0 * std::f64::consts::PI * (t as f64) * (k as f64) / (n as f64);
                sumreal += r * angle.cos() + i * angle.sin();
                sumimag -= r * angle.sin() + i * angle.cos();
            }
            Complex(sumreal, sumimag)
        })
        .collect()
}

pub fn compute_fft(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    let mut output = input.to_vec();

    let mut target = 0;
    for pos in 0..n {
        if target > pos {
            output.swap(target, pos);
        }
        let mut mask = output.len();
        while target & (mask >> 1) != 0 {
            mask >>= 1;
            target &= !mask;
        }
        mask >>= 1;
        target |= mask;
    }

    let mut step = 1;
    while step < n {
        let jump = step << 1;
        let step_d = step as f64;
        let mut twiddle_re = 1.0;
        let mut twiddle_im = 0.0;

        for group in 0..step {
            let mut pair = group;
            while pair < n {
                let match_idx = pair + step;
                let product_re =
                    twiddle_re * output[match_idx].0 - twiddle_im * output[match_idx].1;
                let product_im =
                    twiddle_im * output[match_idx].0 + twiddle_re * output[match_idx].1;

                output[match_idx].0 = output[pair].0 - product_re;
                output[match_idx].1 = output[pair].1 - product_im;
                output[pair].0 += product_re;
                output[pair].1 += product_im;

                pair += jump;
            }

            if group + 1 == step {
                continue;
            }

            let angle = std::f64::consts::PI * ((group + 1) as f64) / step_d;
            twiddle_re = angle.cos();
            twiddle_im = angle.sin();
        }

        step <<= 1;
    }
    output
}

pub fn compute_ifft(input: &[Complex]) -> Vec<Complex> {
    let mut output = input.to_vec();
    for Complex(_, c) in output.iter_mut() {
        *c = -*c;
    }
    let mut result = compute_fft(&output);
    let scale = 1.0 / input.len() as f64;
    for Complex(r, c) in result.iter_mut() {
        *r *= scale;
        *c = -*c * scale;
    }
    result
}

pub fn multiply_polynomials(a: &[f64], b: &[f64]) -> Vec<f64> {
    let n = a.len() + b.len() - 1;
    let size = n.next_power_of_two();

    let mut a = a.to_vec();
    a.resize(size, 0.0);
    let mut b = b.to_vec();
    b.resize(size, 0.0);

    let a = a.iter().map(|&x| Complex(x, 0.0)).collect::<Vec<_>>();
    let b = b.iter().map(|&x| Complex(x, 0.0)).collect::<Vec<_>>();

    let a = compute_fft(&a);
    let b = compute_fft(&b);

    let c: Vec<Complex> = (0..size)
        .map(|i| {
            let re = a[i].0 * b[i].0 - a[i].1 * b[i].1;
            let im = a[i].0 * b[i].1 + a[i].1 * b[i].0;
            Complex(re, im)
        })
        .collect();

    let c = compute_ifft(&c);

    c.iter().take(n).map(|x| x.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_dft_0() {
        let input = vec![
            Complex(0.0, 0.0),
            Complex(1.0, 0.0),
            Complex(0.0, 0.0),
            Complex(1.0, 0.0),
        ];
        let expected = vec![
            Complex(2.0, 0.0),
            Complex(0.0, 0.0),
            Complex(-2.0, 0.0),
            Complex(0.0, 0.0),
        ];
        let output = compute_dft(&input);
        assert_eq!(output, expected);
        let output = compute_fft(&input);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_multiply() {
        // Test polynomials: (x + 1)(x + 2) = x^2 + 3x + 2
        let a = vec![1.0, 1.0]; // x + 1
        let b = vec![2.0, 1.0]; // x + 2
        let result = multiply_polynomials(&a, &b);
        let expected = [2.0, 3.0, 1.0];
        assert_eq!(&result, &expected);
    }
}
