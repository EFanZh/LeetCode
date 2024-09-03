pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    const MODULUS: u32 = 1_000_000_007;

    fn build_matrix(m: u8, result: &mut [u64; 64]) {
        let size = 1_usize << m;
        let result = &mut result[..size * size];
        let mut configuration = u8::MAX;

        for target in result {
            configuration = configuration.wrapping_add(1);

            let mut factor = 0;

            for (mut top, zero_inc) in [(0, false), (0, true), (2, false)] {
                let mut bottom = 1_u8;
                let mut probe = size as u8;

                loop {
                    probe >>= 1;

                    if probe == 0 {
                        factor += 1;

                        break;
                    }

                    top += u8::from((configuration & probe == 0) != zero_inc) + 1;
                    top = top.checked_sub(3).unwrap_or(top);
                    bottom += u8::from(configuration & (probe << m) == 0) + 1;
                    bottom = bottom.checked_sub(3).unwrap_or(bottom);

                    if top == bottom {
                        break;
                    }
                }
            }

            *target = factor;
        }
    }

    fn matrix_multiply(size: usize, lhs: &[u64; 64], rhs: &[u64; 64], result: &mut [u64; 64]) {
        for row in 0..size {
            for column in 0..size {
                let mut value = 0;

                for i in 0..size {
                    value = (value + lhs[size * row + i] * rhs[size * i + column]) % u64::from(Self::MODULUS);
                }

                result[size * row + column] = value;
            }
        }
    }

    fn exp<'a>(
        size: usize,
        mut base: &'a mut [u64; 64],
        mut exponent: u16,
        mut result: &'a mut [u64; 64],
        mut buffer: &'a mut [u64; 64],
    ) -> &'a [u64; 64] {
        for value in result[..size * size].iter_mut().step_by(size + 1) {
            *value = 1;
        }

        if exponent != 0 {
            loop {
                if exponent & 1 != 0 {
                    Self::matrix_multiply(size, result, base, buffer);
                    mem::swap(&mut result, &mut buffer);
                }

                exponent >>= 1;

                if exponent == 0 {
                    break;
                }

                Self::matrix_multiply(size, base, base, buffer);
                mem::swap(&mut base, &mut buffer);
            }
        }

        result
    }

    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let m = m as u8;
        let n = n as u16;

        assert!(m <= 5);

        let mut result = if m < 2 {
            let mut result = 1_u32;

            for _ in 1..n {
                result += result;
                result = result.checked_sub(Self::MODULUS).unwrap_or(result);
            }

            result
        } else {
            let m = m - 2;
            let mut matrix = [0; 64];
            let mut buffer_1 = [0; 64];
            let mut buffer_2 = [0; 64];

            Self::build_matrix(m, &mut matrix);

            let size = 1_usize << m;

            let matrix_exp = Self::exp(size, &mut matrix, n - 1, &mut buffer_1, &mut buffer_2);

            let mut result = 0;

            for &value in &matrix_exp[..size * size] {
                result += value as u32;
                result = result.checked_sub(Self::MODULUS).unwrap_or(result);
            }

            result *= 2;
            result = result.checked_sub(Self::MODULUS).unwrap_or(result);

            result
        };

        result *= 3;
        result = result.checked_sub(Self::MODULUS).unwrap_or(result);
        result = result.checked_sub(Self::MODULUS).unwrap_or(result);

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn color_the_grid(m: i32, n: i32) -> i32 {
        Self::color_the_grid(m, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
