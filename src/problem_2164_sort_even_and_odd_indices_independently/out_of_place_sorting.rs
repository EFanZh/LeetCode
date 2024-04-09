pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut buffer = Vec::<i32>::with_capacity((nums.len() + 1) / 2);

        buffer.extend(nums.iter().step_by(2));
        buffer.sort_unstable();

        for (target, &source) in nums.iter_mut().step_by(2).zip(&buffer) {
            *target = source;
        }

        buffer.clear();

        let nums_1 = &mut nums[1..];

        buffer.extend(nums_1.iter().step_by(2));
        buffer.sort_unstable_by_key(|&num| Reverse(num));

        for (target, &source) in nums_1.iter_mut().step_by(2).zip(&buffer) {
            *target = source;
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_even_odd(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
