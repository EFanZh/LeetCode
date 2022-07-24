pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// See <https://leetcode.com/problems/minimum-cost-to-merge-stones/discuss/247567/JavaC%2B%2BPython-DP>.

use std::num::NonZeroUsize;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let n = stones.len();
        let k = k as usize;
        let k_minus_1 = NonZeroUsize::new(k - 1).unwrap();

        if (n - 1) % k_minus_1 == 0 {
            let mut sums = stones;
            let mut sum = 0;

            for target in &mut sums {
                sum += *target;
                *target = sum;
            }

            let mut cache = vec![0; n * n];

            for length in k..=n {
                for start in 0..=n - length {
                    let mut result = u32::MAX;

                    for split in (1..length).step_by(k_minus_1.get()) {
                        result = result
                            .min(cache[n * (split - 1) + start] + cache[n * (length - split - 1) + start + split]);
                    }

                    if (length - 1) % k_minus_1 == 0 {
                        result +=
                            (sums[start + length - 1] - sums.get(start.wrapping_sub(1)).copied().unwrap_or(0)) as u32;
                    }

                    cache[n * (length - 1) + start] = result;
                }
            }

            cache[n * (n - 1)] as _
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        Self::merge_stones(stones, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
