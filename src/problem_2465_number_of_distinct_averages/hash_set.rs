pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut averages = HashSet::new();

        nums.sort_unstable();

        let mut iter = nums.iter().copied();

        while let (Some(min), Some(max)) = (iter.next(), iter.next_back()) {
            averages.insert((min + max) as u8);
        }

        averages.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distinct_averages(nums: Vec<i32>) -> i32 {
        Self::distinct_averages(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
