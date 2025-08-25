pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as u32;
        let mut start = 0;
        let mut counts = HashMap::new();
        let mut greater_than_k = 0;

        for &num in &nums {
            match counts.entry(num) {
                Entry::Occupied(occupied_entry) => {
                    let count = occupied_entry.into_mut();

                    greater_than_k += u32::from(*count == k);

                    *count += 1;
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
            }

            if greater_than_k != 0 {
                let old = nums[start];

                start += 1;

                let old_count = counts.get_mut(&old).unwrap();

                *old_count -= 1;

                if *old_count == k {
                    greater_than_k -= 1;
                }
            }
        }

        (nums.len() - start) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        Self::max_subarray_length(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
