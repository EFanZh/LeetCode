pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// ⎡ f(n + 1) ⎤   ⎡ 1 << bits(n + 1)  1  1 ⎤ ⎡ f(n) ⎤
// ⎢  n + 1   ⎥ = ⎢        0          1  1 ⎥ ⎢  n   ⎥
// ⎣    0     ⎦   ⎣        0          0  1 ⎦ ⎣  1   ⎦

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn mul(x: (u64, u64, u64, u64), y: (u64, u64, u64, u64)) -> (u64, u64, u64, u64) {
        (
            (x.0 * y.0) % Self::MODULUS,
            (x.0 * y.1 + x.1) % Self::MODULUS,
            (x.0 * y.2 + x.1 * y.3 + x.2) % Self::MODULUS,
            x.3 + y.3,
        )
    }

    fn square(x: (u64, u64, u64, u64)) -> (u64, u64, u64, u64) {
        Self::mul(x, x)
    }

    fn pow(mut base: (u64, u64, u64, u64), mut exponent: u32) -> (u64, u64, u64, u64) {
        let mut result = (1, 0, 0, 0);

        while exponent != 0 {
            if exponent & 1 != 0 {
                result = Self::mul(base, result);
            }

            base = Self::square(base);
            exponent >>= 1;
        }

        result
    }

    fn apply(matrix: (u64, u64, u64, u64), state: (u64, u64)) -> u64 {
        (matrix.0 * state.0 + matrix.1 * state.1 + matrix.2) % Self::MODULUS
    }

    pub fn concatenated_binary(n: i32) -> i32 {
        let n = n as u32;
        let total_bits = 32 - n.leading_zeros();
        let mut result = 0;

        // Concatenate numbers in `1..(1 << (total_bits - 1))`.

        for bits in 1..total_bits {
            let mut matrix = (1 << bits, 1, 1, 1);

            for _ in 1..bits {
                matrix = Self::square(matrix);
            }

            result = Self::apply(matrix, (result, (1 << (bits - 1)) - 1));
        }

        // Concatenate numbers in `(1 << (total_bits - 1))..=n`.

        let matrix = Self::pow((1 << total_bits, 1, 1, 1), n - (1 << (total_bits - 1)) + 1);

        result = Self::apply(matrix, (result, (1 << (total_bits - 1)) - 1));

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn concatenated_binary(n: i32) -> i32 {
        Self::concatenated_binary(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
