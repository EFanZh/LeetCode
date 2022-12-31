pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as _;
        let mut result = Vec::with_capacity(nums.len() - (k - 1));
        let mut queue = VecDeque::with_capacity(k - 1);
        let (left, right) = nums.split_at(k);

        for &num in left {
            while queue.back().map_or(false, |&back| back < num) {
                queue.pop_back();
            }

            queue.push_back(num);
        }

        result.push(queue[0]);

        for (&old, &num) in nums.iter().zip(right) {
            if queue[0] == old {
                queue.pop_front();
            }

            while queue.back().map_or(false, |&back| back < num) {
                queue.pop_back();
            }

            queue.push_back(num);
            result.push(queue[0]);
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
