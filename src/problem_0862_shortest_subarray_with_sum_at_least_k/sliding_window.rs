pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let k = i64::from(k);
        let mut result = u32::MAX;
        let mut sum = 0;
        let mut queue = VecDeque::from(vec![(0, 0)]);

        for (i, &num) in (1..).zip(&nums) {
            sum += i64::from(num);

            while let Some(&(_, value)) = queue.back() {
                if value >= sum {
                    queue.pop_back();
                } else {
                    break;
                }
            }

            let target = sum - k;

            while let Some(&(j, value)) = queue.front() {
                if value <= target {
                    queue.pop_front();

                    result = result.min(i - j);
                } else {
                    break;
                }
            }

            queue.push_back((i, sum));
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        Self::shortest_subarray(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
