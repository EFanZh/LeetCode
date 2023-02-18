pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    const MODULUS: u32 = 1_000_000_007;

    fn subtract(lhs: u32, rhs: u32) -> u32 {
        lhs + Self::MODULUS - rhs
    }

    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let (n, m, k) = (n as usize, m as usize, k as usize);
        let cache_size = m * k;
        let mut buffer = vec![0; cache_size * 2];
        let (mut cache, mut temp) = buffer.split_at_mut(cache_size);

        // Length = 1.

        for (target, max) in cache[..m].iter_mut().zip(1..) {
            *target = max;
        }

        // Length > 1.

        for length in 2..=n {
            // Cost = 1.

            {
                let mut count = 0;

                for max in 1..=m {
                    // Not increasing cost.
                    count += u64::from(Self::subtract(
                        cache[max - 1],
                        cache.get(max.wrapping_sub(2)).copied().unwrap_or(0),
                    )) * (max as u64);

                    // Normalize result.
                    count %= u64::from(Self::MODULUS);

                    temp[max - 1] = count as u32;
                }
            }

            // Cost > 1.

            for cost in 2..=k.min(length) {
                let mut count = 0;

                for max in cost..=m {
                    // Not increasing cost.
                    count += u64::from(Self::subtract(
                        cache[m * (cost - 1) + max - 1],
                        cache[m * (cost - 1) + max - 2],
                    )) * (max as u64);

                    // Increasing cost.
                    count += u64::from(cache[m * (cost - 2) + max - 2]);

                    // Normalize result.
                    count %= u64::from(Self::MODULUS);

                    temp[m * (cost - 1) + max - 1] = count as u32;
                }
            }

            mem::swap(&mut cache, &mut temp);
        }

        *cache.last().unwrap() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        Self::num_of_arrays(n, m, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
