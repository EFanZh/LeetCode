pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;

        nums.sort_unstable_by_key(|&num| Reverse(num as u32));

        let mut result = 0;
        let mut prev = 0;

        for (count, num) in (0..).zip(nums) {
            if num != prev {
                result += count;
                prev = num;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reduction_operations(nums: Vec<i32>) -> i32 {
        Self::reduction_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
