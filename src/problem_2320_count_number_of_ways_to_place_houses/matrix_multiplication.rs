pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

type Matrix = (u64, u64);

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn matrix_multiply(lhs: Matrix, rhs: Matrix) -> Matrix {
        (
            (lhs.0 * (rhs.0 + rhs.1) + lhs.1 * rhs.0) % Self::MODULUS,
            (lhs.0 * rhs.0 + lhs.1 * rhs.1) % Self::MODULUS,
        )
    }

    fn exp_mod(mut exponent: u32) -> Matrix {
        let mut base = (1, 0);
        let mut result = (0, 1);

        loop {
            if exponent & 1 != 0 {
                result = Self::matrix_multiply(result, base);
            }

            exponent >>= 1;

            if exponent == 0 {
                break;
            }

            base = Self::matrix_multiply(base, base);
        }

        result
    }

    pub fn count_house_placements(n: i32) -> i32 {
        let matrix = Self::exp_mod(n as _);

        ((matrix.0 * 2 + matrix.1).pow(2) % Self::MODULUS) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_house_placements(n: i32) -> i32 {
        Self::count_house_placements(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
