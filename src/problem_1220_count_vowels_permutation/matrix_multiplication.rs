pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn matrix_multiply(lhs: &[[u64; 5]; 5], rhs: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
        let mut result = [[0; 5]; 5];

        for (target_row, lhs_row) in result.iter_mut().zip(lhs) {
            for (i, target) in target_row.iter_mut().enumerate() {
                for (lhs_value, rhs_row) in lhs_row.iter().zip(rhs) {
                    *target += lhs_value * rhs_row[i];
                }

                *target %= Self::MODULUS;
            }
        }

        result
    }

    pub fn count_vowel_permutation(n: i32) -> i32 {
        let base = [
            [0, 1, 0, 0, 0],
            [1, 0, 1, 0, 0],
            [1, 1, 0, 1, 1],
            [0, 0, 1, 0, 1],
            [1, 0, 0, 0, 0],
        ];

        let mut matrix = [
            [1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 0, 1],
        ];

        let n = n - 1;

        for i in (0..32 - n.leading_zeros()).rev() {
            matrix = Self::matrix_multiply(&matrix, &matrix);

            if n & (1 << i) != 0 {
                matrix = Self::matrix_multiply(&matrix, &base);
            }
        }

        (matrix.iter().map(|row| row.iter().sum::<u64>()).sum::<u64>() % Self::MODULUS) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_vowel_permutation(n: i32) -> i32 {
        Self::count_vowel_permutation(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
