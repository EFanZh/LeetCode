pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn pow_mod(mut base: u32, mut exponent: u32, modulus: u32) -> u32 {
        let mut result = 1;

        while exponent != 0 {
            if exponent & 1 != 0 {
                result = (result * base) % modulus;
            }

            exponent >>= 1;
            base = (base * base) % modulus;
        }

        result
    }

    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        const M: u32 = 1337;

        let a = a as u32 % M;
        let mut result = 1;

        for digit in b.into_iter().map(|d| d as _) {
            result = (Self::pow_mod(result, 10, M) * Self::pow_mod(a, digit, M)) % M;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        Self::super_pow(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
