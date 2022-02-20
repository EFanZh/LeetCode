pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

const MODULUS: u64 = 1_000_000_007;

impl Solution {
    fn matrix_multiply(lhs: &[[u64; 4]; 4], rhs: &[[u64; 4]; 4]) -> [[u64; 4]; 4] {
        let mut result = [[0; 4]; 4];

        for (target_row, row) in result.iter_mut().zip(lhs) {
            for (i, target) in target_row.iter_mut().enumerate() {
                let mut value = 0;

                for (lhs, rhs_row) in row.iter().zip(rhs) {
                    value += lhs * rhs_row[i];
                }

                *target = value;
            }
        }

        result
    }

    fn get_matrix(n: u32) -> [[u64; 4]; 4] {
        // 0 1 0
        // 2   2
        // 0 1 0
        //   3

        let matrix = [[0, 1, 1, 0], [2, 0, 0, 0], [2, 0, 0, 1], [0, 0, 2, 0]];

        let mut result = matrix;

        for i in (0..31 - n.leading_zeros()).rev() {
            result = Self::matrix_multiply(&result, &result);

            if n & (1 << i) != 0 {
                result = Self::matrix_multiply(&result, &matrix);
            }

            for row in &mut result {
                for value in row {
                    *value %= MODULUS;
                }
            }
        }

        result
    }

    pub fn knight_dialer(n: i32) -> i32 {
        let n = n as u32;

        if n == 1 {
            10
        } else {
            let matrix = Self::get_matrix(n - 1);
            let sum = |i: usize| matrix[i].iter().sum::<u64>();
            let result = sum(0) * 4 + sum(1) * 2 + sum(2) * 2 + sum(3);

            (result % MODULUS) as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn knight_dialer(n: i32) -> i32 {
        Self::knight_dialer(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
