pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let (&first, rest) = nums.split_first().unwrap();
        let mut sum_set = HashSet::with_capacity(nums.len().min(k as _));
        let mut sum = first % k;

        sum_set.insert(0);

        for num in rest {
            let prev = sum;

            sum += num;
            sum %= k;

            if sum_set.contains(&sum) {
                return true;
            }

            sum_set.insert(prev);
        }

        false
    }
}

impl super::Solution for Solution {
    fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        Self::check_subarray_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
