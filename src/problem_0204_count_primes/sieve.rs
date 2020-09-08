pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            0
        } else {
            let mut candidates = vec![false; ((n - 2) / 2) as _];

            for (mut i, num) in (3..=(f64::from(n - 1).sqrt() as _)).step_by(2).enumerate() {
                if !candidates[i] {
                    i += num;

                    while let Some(slot) = candidates.get_mut(i) {
                        *slot = true;
                        i += num;
                    }
                }
            }

            let mut result = 1;

            for value in candidates {
                if !value {
                    result += 1;
                }
            }

            result
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
