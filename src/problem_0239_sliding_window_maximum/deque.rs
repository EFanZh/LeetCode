pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as _;
        let mut result = Vec::with_capacity(nums.len() - k + 1);

        if !nums.is_empty() {
            let mut stack = VecDeque::with_capacity(k);
            let (nums_1, nums_2) = nums.split_at(k);

            for num in nums_1 {
                if let Some(i) = stack.iter().position(|x| x < num) {
                    stack.truncate(i);
                }

                stack.push_back(*num);
            }

            for (new_num, old_num) in nums_2.iter().zip(&nums) {
                let front = stack[0];

                result.push(front);

                if front == *old_num {
                    stack.pop_front();
                }

                if let Some(i) = stack.iter().position(|x| x < new_num) {
                    stack.truncate(i);
                }

                stack.push_back(*new_num);
            }

            result.push(stack[0]);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::max_sliding_window(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
