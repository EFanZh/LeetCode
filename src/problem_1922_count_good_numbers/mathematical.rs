pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn exp_mod(mut base: u64, mut exponent: u64) -> u64 {
        let mut result = 1;

        while exponent != 0 {
            if exponent & 1 != 0 {
                result = result * base % Self::MODULUS;
            }

            base = base * base % Self::MODULUS;
            exponent >>= 1;
        }

        result
    }

    pub fn count_good_numbers(n: i64) -> i32 {
        let n = n as u64;
        let even_count = Self::exp_mod(5, (n + 1) / 2);
        let odd_count = Self::exp_mod(4, n / 2);

        ((even_count * odd_count) % Self::MODULUS) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_good_numbers(n: i64) -> i32 {
        Self::count_good_numbers(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
