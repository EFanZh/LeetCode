pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let threshold = threshold as u32;
        let mut left = 1;
        let mut right = nums.iter().map(|&value| value as u32).max().unwrap();

        while left < right {
            let middle = NonZeroU32::new((left + right) / 2).unwrap();
            let middle_minus_1 = middle.get() - 1;

            if nums
                .iter()
                .map(|&value| (value as u32 + middle_minus_1) / middle)
                .sum::<u32>()
                <= threshold
            {
                right = middle.get();
            } else {
                left = middle.get() + 1;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        Self::smallest_divisor(nums, threshold)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
