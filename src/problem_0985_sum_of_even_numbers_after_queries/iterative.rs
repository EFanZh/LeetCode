pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut sum = nums.iter().filter(|&num| num % 2 == 0).sum::<i32>();

        queries
            .iter()
            .map(|query| {
                let [value, index]: [_; 2] = query.as_slice().try_into().unwrap();
                let target = &mut nums[index as usize];
                let old_target = *target;

                *target += value;

                let new_target = *target;

                match (old_target % 2 == 0, new_target % 2 == 0) {
                    (false, false) => {}
                    (false, true) => sum += new_target,
                    (true, false) => sum -= old_target,
                    (true, true) => sum += value,
                }

                sum
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::sum_even_after_queries(nums, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
