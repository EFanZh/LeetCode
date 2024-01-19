pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut visited = vec![false; 1001].into_boxed_slice();
        let mut queue = VecDeque::from([start]);
        let mut result = 1;

        visited[start as u32 as usize] = true;

        loop {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();

                for &num in &nums {
                    for next in [current + num, current - num, current ^ num] {
                        if next == goal {
                            return result;
                        }

                        if let Some(state @ false) = visited.get_mut(next as usize) {
                            *state = true;

                            queue.push_back(next);
                        }
                    }
                }
            }

            if queue.is_empty() {
                break;
            }

            result += 1;
        }

        -1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        Self::minimum_operations(nums, start, goal)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
