pub struct Solution {}

use std::ops::BitXor;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, BitXor::bitxor)
    }
}

impl super::Solution for Solution {
    fn single_number(nums: Vec<i32>) -> i32 {
        Self::single_number(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
