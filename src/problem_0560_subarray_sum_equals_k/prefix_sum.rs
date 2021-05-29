pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut prefix_sums = HashMap::with_capacity(nums.len());
        let mut sum = 0;

        for num in nums {
            prefix_sums.entry(sum).and_modify(|c| *c += 1).or_insert(1);
            sum += num;

            if let Some(count) = prefix_sums.get(&(sum - k)) {
                result += count;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarray_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
