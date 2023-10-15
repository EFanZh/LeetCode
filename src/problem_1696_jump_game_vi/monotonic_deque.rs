pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let mut queue = VecDeque::<(u32, i32)>::new();
        let mut current_cost = 0;

        for (i, cost) in (0_u32..).zip(nums) {
            current_cost = loop {
                if let Some(&(left_index, left_cost)) = queue.front() {
                    if left_index + k >= i {
                        break left_cost + cost;
                    }

                    queue.pop_front();
                } else {
                    break cost;
                }
            };

            while queue.back().map_or(false, |back| back.1 <= current_cost) {
                queue.pop_back();
            }

            queue.push_back((i, current_cost));
        }

        current_cost
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        Self::max_result(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
