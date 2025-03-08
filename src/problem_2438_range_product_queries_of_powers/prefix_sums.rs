pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u64 = 1_000_000_007;

    fn exp_mod(mut base: u64, mut exponent: u32) -> u64 {
        let mut result = 1;

        loop {
            if exponent & 1 != 0 {
                result = (result * base) % Self::MODULUS;
            }

            exponent >>= 1;

            if exponent == 0 {
                break;
            }

            base = (base * base) % Self::MODULUS;
        }

        result
    }

    fn mod_inverse(n: u64) -> u64 {
        Self::exp_mod(n, Self::MODULUS as u32 - 2)
    }

    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut n = n as u32;
        let mut powers = [0_u64; 32];
        let mut power_inversions = [0_u64; 32];
        let mut product = 1;
        let mut i = 0;

        while n != 0 {
            power_inversions[i] = Self::mod_inverse(product);

            let last_bit = n & n.wrapping_neg();

            product = (product * u64::from(last_bit)) % Self::MODULUS;
            n ^= last_bit;
            powers[i] = product;
            i += 1;
        }

        queries
            .into_iter()
            .map(|query| {
                let [left, right] = query.try_into().ok().unwrap();
                let high = powers[right as u32 as usize];
                let low = power_inversions[left as u32 as usize];

                ((high * low) % Self::MODULUS) as _
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::product_queries(n, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
