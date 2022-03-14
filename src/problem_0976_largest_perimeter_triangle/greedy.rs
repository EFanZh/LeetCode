pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = BinaryHeap::from(nums);
        let mut prev_1 = nums.pop().unwrap();
        let mut prev_2 = nums.pop().unwrap();

        while let Some(num) = nums.pop() {
            if num > prev_1 - prev_2 {
                return prev_1 + prev_2 + num;
            }

            prev_1 = prev_2;
            prev_2 = num;
        }

        0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_perimeter(nums: Vec<i32>) -> i32 {
        Self::largest_perimeter(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
