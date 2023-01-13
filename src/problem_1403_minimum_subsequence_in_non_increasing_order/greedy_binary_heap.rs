pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let target = nums.iter().sum::<i32>() / 2;
        let mut queue = BinaryHeap::from(nums);
        let mut sum = 0;
        let mut result = Vec::new();

        loop {
            let top = queue.pop().unwrap();

            sum += top;
            result.push(top);

            if sum > target {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        Self::min_subsequence(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
