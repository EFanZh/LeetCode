pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

const MODULUS: u64 = 1_000_000_007;

type Matrix = ((u64, u64, u64), (u64, u64, u64), (u64, u64, u64));

impl Solution {
    fn matrix_multiply(lhs: Matrix, rhs: Matrix) -> Matrix {
        (
            (
                (lhs.0 .0 * rhs.0 .0 + lhs.0 .1 * rhs.1 .0 + lhs.0 .2 * rhs.2 .0),
                (lhs.0 .0 * rhs.0 .1 + lhs.0 .1 * rhs.1 .1 + lhs.0 .2 * rhs.2 .1),
                (lhs.0 .0 * rhs.0 .2 + lhs.0 .1 * rhs.1 .2 + lhs.0 .2 * rhs.2 .2),
            ),
            (
                (lhs.1 .0 * rhs.0 .0 + lhs.1 .1 * rhs.1 .0 + lhs.1 .2 * rhs.2 .0),
                (lhs.1 .0 * rhs.0 .1 + lhs.1 .1 * rhs.1 .1 + lhs.1 .2 * rhs.2 .1),
                (lhs.1 .0 * rhs.0 .2 + lhs.1 .1 * rhs.1 .2 + lhs.1 .2 * rhs.2 .2),
            ),
            (
                (lhs.2 .0 * rhs.0 .0 + lhs.2 .1 * rhs.1 .0 + lhs.2 .2 * rhs.2 .0),
                (lhs.2 .0 * rhs.0 .1 + lhs.2 .1 * rhs.1 .1 + lhs.2 .2 * rhs.2 .1),
                (lhs.2 .0 * rhs.0 .2 + lhs.2 .1 * rhs.1 .2 + lhs.2 .2 * rhs.2 .2),
            ),
        )
    }

    fn get_matrix(n: u32) -> Matrix {
        let matrix = ((0, 1, 0), (1, 1, 2), (1, 0, 1));

        match n {
            0 => ((1, 0, 0), (0, 1, 0), (0, 1, 0)),
            1 => matrix,
            n => {
                let mut result = matrix;
                let mut probe = 1 << (30 - n.leading_zeros());

                loop {
                    result = Self::matrix_multiply(result, result);

                    if n & probe != 0 {
                        result = Self::matrix_multiply(result, matrix);
                    }

                    result.0 .0 %= MODULUS;
                    result.0 .1 %= MODULUS;
                    result.0 .2 %= MODULUS;
                    result.1 .0 %= MODULUS;
                    result.1 .1 %= MODULUS;
                    result.1 .2 %= MODULUS;
                    result.2 .0 %= MODULUS;
                    result.2 .1 %= MODULUS;
                    result.2 .2 %= MODULUS;

                    probe >>= 1;

                    if probe == 0 {
                        break;
                    }
                }

                result
            }
        }
    }

    pub fn num_tilings(n: i32) -> i32 {
        let matrix = Self::get_matrix(n as u32 - 1);

        ((matrix.1 .0 + matrix.1 .1) % MODULUS) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_tilings(n: i32) -> i32 {
        Self::num_tilings(n)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
