pub struct Solution;

impl Solution {
    fn is_prime(i: i32, nums: &[i32]) -> bool {
        for num in nums {
            if i % num == 0 {
                return false;
            }
        }

        true
    }

    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            0
        } else {
            let mut primes_except_2 = Vec::new();

            for i in (3..n).step_by(2) {
                let j = match primes_except_2.binary_search(&(f64::from(i).sqrt() as _)) {
                    Ok(j) => j + 1,
                    Err(j) => j,
                };

                if Self::is_prime(i, &primes_except_2[..j]) {
                    primes_except_2.push(i);
                }
            }

            (primes_except_2.len() as i32) + 1
        }
    }
}

impl super::Solution for Solution {
    fn count_primes(n: i32) -> i32 {
        Self::count_primes(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
