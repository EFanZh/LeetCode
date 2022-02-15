pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let goal = goal as u32 as usize;
        let mut queue = VecDeque::with_capacity(goal);
        let mut iter = (0_u32..).zip(nums);

        while queue.len() < goal {
            loop {
                if let Some((i, num)) = iter.next() {
                    if num != 0 {
                        queue.push_back(i);

                        break;
                    }
                } else {
                    return 0;
                }
            }
        }

        if let Some(&leading_zeros) = queue.front() {
            let mut factor = leading_zeros + 1;
            let mut result = factor;

            for (i, num) in iter {
                if num != 0 {
                    let removed = queue.pop_front().unwrap();

                    queue.push_back(i);

                    factor = *queue.front().unwrap() - removed;
                }

                result += factor;
            }

            result as _
        } else {
            let mut result = 0;
            let mut prev_1 = u32::MAX;

            for (i, num) in iter {
                if num == 0 {
                    result += i.wrapping_sub(prev_1);
                } else {
                    prev_1 = i;
                }
            }

            result as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        Self::num_subarrays_with_sum(nums, goal)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
