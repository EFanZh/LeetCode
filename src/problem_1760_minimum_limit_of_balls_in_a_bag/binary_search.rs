pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    fn check(nums: &[i32], penalty: u32, mut max_operations: u32) -> bool {
        let penalty = NonZeroU32::new(penalty).unwrap();

        nums.iter().all(|&num| {
            let required_ops = (num as u32 - 1) / penalty;
            let ok = required_ops <= max_operations;

            if ok {
                max_operations -= required_ops;
            }

            ok
        })
    }

    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let max_operations = max_operations as u32;
        let mut left = 1;
        let mut right = nums.iter().fold(0, |max, &num| max.max(num as _));

        loop {
            let middle = (left + right) >> 1;

            if Self::check(&nums, middle, max_operations) {
                right = middle;
            } else {
                left = middle + 1;
            }

            if left == right {
                return left as _;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        Self::minimum_size(nums, max_operations)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
