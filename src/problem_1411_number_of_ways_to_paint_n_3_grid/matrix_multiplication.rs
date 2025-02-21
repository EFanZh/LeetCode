pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

const MODULUS: u64 = 1_000_000_007;

impl Solution {
    fn matrix_multiplication(lhs: ((u64, u64), (u64, u64)), rhs: ((u64, u64), (u64, u64))) -> ((u64, u64), (u64, u64)) {
        (
            (
                (lhs.0.0 * rhs.0.0 + lhs.0.1 * rhs.1.0) % MODULUS,
                (lhs.0.0 * rhs.0.1 + lhs.0.1 * rhs.1.1) % MODULUS,
            ),
            (
                (lhs.1.0 * rhs.0.0 + lhs.1.1 * rhs.1.0) % MODULUS,
                (lhs.1.0 * rhs.0.1 + lhs.1.1 * rhs.1.1) % MODULUS,
            ),
        )
    }

    pub fn num_of_ways(n: i32) -> i32 {
        let mut matrix = ((1_u64, 0_u64), (0_u64, 1_u64));
        let n = n as u32 - 1;
        let mut probe = (1 << (32 - n.leading_zeros())) >> 1;

        while probe != 0 {
            matrix = Self::matrix_multiplication(matrix, matrix);

            if n & probe != 0 {
                matrix = Self::matrix_multiplication(matrix, ((3, 2), (2, 2)));
            }

            probe >>= 1;
        }

        (((matrix.0.0 + matrix.0.1 + matrix.1.0 + matrix.1.1) * 6) % MODULUS) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_ways(n: i32) -> i32 {
        Self::num_of_ways(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
