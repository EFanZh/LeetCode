pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn exp_mod(mut base: u64, mut exponent: u64) -> u64 {
        let mut result = 1;

        while exponent != 0 {
            if exponent & 1 != 0 {
                result = (result * base) % Self::MODULUS;
            }

            base = (base * base) % Self::MODULUS;
            exponent >>= 1;
        }

        result
    }

    pub fn max_nice_divisors(prime_factors: i32) -> i32 {
        let prime_factors = prime_factors as u32;

        (if prime_factors < 5 {
            prime_factors
        } else {
            let prime_factors_div_3 = u64::from(prime_factors / 3);

            (match prime_factors % 3 {
                0 => Self::exp_mod(3, prime_factors_div_3),
                1 => (Self::exp_mod(3, prime_factors_div_3 - 1) * 4) % Self::MODULUS,
                _ => (Self::exp_mod(3, prime_factors_div_3) * 2) % Self::MODULUS,
            }) as _
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_nice_divisors(prime_factors: i32) -> i32 {
        Self::max_nice_divisors(prime_factors)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
