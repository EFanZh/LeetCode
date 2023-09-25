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

    // See <https://en.wikipedia.org/wiki/Modular_multiplicative_inverse#Using_Euler's_theorem>.
    fn mod_inv(x: u64) -> u64 {
        Self::exp_mod(x, Self::MODULUS - 2)
    }

    fn combinations(n: u64, k: u64) -> u64 {
        let n_minus_k = n - k;
        let (k, n_minus_k) = if n_minus_k < k { (n_minus_k, k) } else { (k, n_minus_k) };
        let mut factorial = 1;

        for i in 2..=k {
            factorial = (factorial * i) % Self::MODULUS;
        }

        let factorial_k = factorial;

        for i in k + 1..=n_minus_k {
            factorial = (factorial * i) % Self::MODULUS;
        }

        let factorial_n_minus_k = factorial;

        for i in n_minus_k + 1..=n {
            factorial = (factorial * i) % Self::MODULUS;
        }

        let factorial_n = factorial;

        (factorial_n * Self::mod_inv((factorial_k * factorial_n_minus_k) % Self::MODULUS)) % Self::MODULUS
    }

    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        let n = u64::from(n as u32);
        let k = u64::from(k as u32);

        Self::combinations(n + k - 1, k * 2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_sets(n: i32, k: i32) -> i32 {
        Self::number_of_sets(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
