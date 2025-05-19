pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    fn check(candies: &[i32], mut k: u64, candidate: u32) -> bool {
        let candidate = NonZeroU32::new(candidate).unwrap();

        for &candies in candies {
            let satisfied = u64::from(candies as u32 / candidate);

            if satisfied >= k {
                return true;
            }

            k -= satisfied;
        }

        false
    }

    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let candies = candies.as_slice();
        let k = k as u64;
        let mut left = 1;
        let mut right = candies.iter().fold(0, |acc, &x| u32::max(acc, x as _)) + 1;

        while left < right {
            let middle = u32::midpoint(left, right);

            if Self::check(candies, k, middle) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        (left - 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        Self::maximum_candies(candies, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
