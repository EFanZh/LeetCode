pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ops::{BitOrAssign, Shl};

impl Solution {
    fn check<T>(zero: T, one: T, iter: impl IntoIterator<Item = i32>) -> T
    where
        T: BitOrAssign + Copy + Shl<u32, Output = T>,
    {
        let mut bits = zero;

        for x in iter {
            bits |= one << (x as u32 - 1);
        }

        bits
    }

    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();

        if n < 65 {
            let full = if n < 64 { u64::wrapping_shl(1, n as _) } else { 0 }.wrapping_sub(1);

            for row in &matrix {
                if Self::check(0, 1, row.iter().copied()) != full {
                    return false;
                }
            }

            for column in 0..n {
                if Self::check(0, 1, matrix.iter().map(|row| row[column])) != full {
                    return false;
                }
            }
        } else {
            let full = (1_u128 << n).wrapping_sub(1);

            for row in &matrix {
                if Self::check(0, 1, row.iter().copied()) != full {
                    return false;
                }
            }

            for column in 0..n {
                if Self::check(0, 1, matrix.iter().map(|row| row[column])) != full {
                    return false;
                }
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        Self::check_valid(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
