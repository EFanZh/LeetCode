pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let nums = HashSet::<i32>::from_iter(nums);
        let mut result = original;

        while nums.contains(&result) {
            result *= 2;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        Self::find_final_value(nums, original)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
