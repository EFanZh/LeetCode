pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::mem;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut num_to_sums = HashMap::<u16, u32>::new();
        let mut sum = 0;
        let mut left_sum = 0;
        let mut result = 0;

        for &num in &nums {
            sum += num as u32;

            match num_to_sums.entry(num as _) {
                Entry::Occupied(entry) => left_sum = left_sum.max(mem::replace(entry.into_mut(), sum)),
                Entry::Vacant(entry) => {
                    entry.insert(sum);
                }
            }

            result = result.max(sum - left_sum);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        Self::maximum_unique_subarray(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
