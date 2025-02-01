pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let nums = HashSet::<_>::from_iter(nums);

        nums.iter()
            .copied()
            .filter(|&num| num > 0 && nums.contains(&-num))
            .max()
            .unwrap_or(-1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_max_k(nums: Vec<i32>) -> i32 {
        Self::find_max_k(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
