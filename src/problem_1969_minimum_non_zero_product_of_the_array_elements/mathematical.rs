pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn mul(lhs: u64, rhs: u64) -> u64 {
        (lhs * rhs) % Self::MODULUS
    }

    /// Calculates `base.pow(2.pow(x) - 1) % Self::MODULUS`.
    fn exp(mut base: u64, x: u32) -> u64 {
        let mut result = 1;

        for _ in 0..x {
            result = Self::mul(result, base);
            base = Self::mul(base, base);
        }

        result
    }

    pub fn min_non_zero_product(p: i32) -> i32 {
        let p = p as u32;
        let max = (1 << p) - 1;

        Self::mul(max % Self::MODULUS, Self::exp((max - 1) % Self::MODULUS, p - 1)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_non_zero_product(p: i32) -> i32 {
        Self::min_non_zero_product(p)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
