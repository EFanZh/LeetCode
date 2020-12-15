pub struct Solution;

impl Solution {
    fn pow_mod(base: u32, exponent: u32, modulus: u32) -> u32 {
        let mut result = 1;

        for i in (0..32 - exponent.leading_zeros()).rev() {
            if exponent & (1 << i) == 0 {
                result = (result * result) % modulus;
            } else {
                result = (result * result * base) % modulus;
            }
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
