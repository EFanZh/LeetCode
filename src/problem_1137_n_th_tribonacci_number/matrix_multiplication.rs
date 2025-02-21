pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

type Matrix = ((u32, u32, u32), (u32, u32, u32), (u32, u32, u32));

impl Solution {
    fn matrix_multiply(lhs: Matrix, rhs: Matrix) -> Matrix {
        (
            (
                (lhs.0.0 * rhs.0.0 + lhs.0.1 * rhs.1.0 + lhs.0.2 * rhs.2.0),
                (lhs.0.0 * rhs.0.1 + lhs.0.1 * rhs.1.1 + lhs.0.2 * rhs.2.1),
                (lhs.0.0 * rhs.0.2 + lhs.0.1 * rhs.1.2 + lhs.0.2 * rhs.2.2),
            ),
            (
                (lhs.1.0 * rhs.0.0 + lhs.1.1 * rhs.1.0 + lhs.1.2 * rhs.2.0),
                (lhs.1.0 * rhs.0.1 + lhs.1.1 * rhs.1.1 + lhs.1.2 * rhs.2.1),
                (lhs.1.0 * rhs.0.2 + lhs.1.1 * rhs.1.2 + lhs.1.2 * rhs.2.2),
            ),
            (
                (lhs.2.0 * rhs.0.0 + lhs.2.1 * rhs.1.0 + lhs.2.2 * rhs.2.0),
                (lhs.2.0 * rhs.0.1 + lhs.2.1 * rhs.1.1 + lhs.2.2 * rhs.2.1),
                (lhs.2.0 * rhs.0.2 + lhs.2.1 * rhs.1.2 + lhs.2.2 * rhs.2.2),
            ),
        )
    }

    fn get_matrix(n: u32) -> Matrix {
        let matrix = ((0, 1, 0), (0, 0, 1), (1, 1, 1));

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

                    probe >>= 1;

                    if probe == 0 {
                        break;
                    }
                }

                result
            }
        }
    }

    pub fn tribonacci(n: i32) -> i32 {
        let matrix = Self::get_matrix(n as _);

        (matrix.0.1 + matrix.0.2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn tribonacci(n: i32) -> i32 {
        Self::tribonacci(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
