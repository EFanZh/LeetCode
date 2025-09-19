pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut num_to_prefix_sums = HashMap::<i32, i64>::new();
        let mut prefix_sum = 0;
        let mut result = i64::MIN;

        for num in nums {
            match num_to_prefix_sums.entry(num) {
                Entry::Occupied(occupied_entry) => {
                    let sum = occupied_entry.into_mut();

                    *sum = (*sum).min(prefix_sum);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(prefix_sum);
                }
            }

            prefix_sum += i64::from(num);

            for key in [num - k, num + k] {
                if let Some(&sum) = num_to_prefix_sums.get(&key) {
                    result = result.max(prefix_sum - sum);
                }
            }
        }

        if result == i64::MIN { 0 } else { result }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        Self::maximum_subarray_sum(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
