pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::array;

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn matrix_multiply(lhs: &[[u32; 26]; 26], rhs: &[[u32; 26]; 26]) -> [[u32; 26]; 26] {
        let mut result = [[0; 26]; 26];

        result.iter_mut().zip(lhs).for_each(|(target_row, lhs_row)| {
            for (i, target) in target_row.iter_mut().enumerate() {
                *target = lhs_row.iter().zip(rhs).fold(0, |sum, (&lhs_value, rhs_row)| {
                    (sum + u64::from(lhs_value) * u64::from(rhs_row[i])) % Self::MODULUS
                }) as _;
            }
        });

        result
    }

    fn exp_mod(mut base: [[u32; 26]; 26], mut exponent: u32) -> [[u32; 26]; 26] {
        while exponent & 1 == 0 {
            base = Self::matrix_multiply(&base, &base);
            exponent >>= 1;
        }

        if exponent == 1 {
            return base;
        }

        let mut result = base;

        loop {
            exponent >>= 1;
            base = Self::matrix_multiply(&base, &base);

            if exponent & 1 != 0 {
                result = Self::matrix_multiply(&result, &base);

                if exponent == 1 {
                    break;
                }
            }
        }

        result
    }

    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let base = array::from_fn(|i| {
            let mut result = [0; 26];

            result[25] = u32::from(i < 2);

            if i != 0 {
                result[i - 1] = 1;
            }

            result
        });

        let matrix = Self::exp_mod(base, t.cast_unsigned());

        let mut counts = [0_u32; 26];

        for c in s.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        matrix.iter().fold(0, |sum, row| {
            row.iter().zip(&counts).fold(sum, |sum, (&lhs, &rhs)| {
                (sum + u64::from(lhs) * u64::from(rhs)) % Self::MODULUS
            })
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn length_after_transformations(s: String, t: i32) -> i32 {
        Self::length_after_transformations(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
