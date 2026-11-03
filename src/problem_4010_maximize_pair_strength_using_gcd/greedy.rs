pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    fn gcd(mut x: NonZero<u32>, mut y: u32) -> NonZero<u32> {
        while let Some(z) = NonZero::new(y % x) {
            (y, x) = (x.get(), z);
        }

        x
    }

    pub fn max_pair_strength(nums: Vec<i32>) -> i64 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();

        nums.sort_unstable();
        nums.dedup();

        let max = u64::from(*nums.last().unwrap());
        let mut result = 1;

        for i in (0..nums.len()).rev() {
            let mut iter = nums[i..].iter().copied();
            let left = iter.next().and_then(NonZero::new).unwrap();

            if u64::from(left.get()) * max > result {
                for right in iter {
                    let gcd = Self::gcd(left, right);

                    result = result.max(u64::from(left.get() / gcd) * u64::from(right / gcd));
                }
            } else {
                break;
            }
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_pair_strength(nums: Vec<i32>) -> i64 {
        Self::max_pair_strength(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
