pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut min_stack = VecDeque::new();
        let mut max_stack = VecDeque::new();
        let mut left_iter = nums.iter().copied();
        let mut result = 0;

        for &num in &nums {
            while min_stack.back().is_some_and(|&item| item > num) {
                min_stack.pop_back();
            }

            min_stack.push_back(num);

            while max_stack.back().is_some_and(|&item| item < num) {
                max_stack.pop_back();
            }

            max_stack.push_back(num);

            let max = max_stack[0];
            let min = min_stack[0];

            if max - min > limit {
                let old = left_iter.next().unwrap();

                if old == min {
                    min_stack.pop_front();
                }

                if old == max {
                    max_stack.pop_front();
                }
            } else {
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        Self::longest_subarray(nums, limit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
