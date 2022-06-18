pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See: <https://leetcode.com/problems/prime-arrangements/discuss/439131/on-the-fly>.

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        let n = n as usize;
        let mut is_composite = vec![false; n + 1];
        let mut primes = 0_u64;
        let mut composites = 1_u64;
        let mut result = 1_u64;

        for i in 2..=n {
            let value = if is_composite[i] {
                composites += 1;

                composites
            } else {
                let mut j = i * i;

                while let Some(value) = is_composite.get_mut(j) {
                    *value = true;

                    j += i;
                }

                primes += 1;

                primes
            };

            result = (result * value) % 1_000_000_007;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_prime_arrangements(n: i32) -> i32 {
        Self::num_prime_arrangements(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
