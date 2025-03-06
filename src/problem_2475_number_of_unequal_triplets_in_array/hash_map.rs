pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        let n = nums.len() as u32;

        for num in nums {
            match counts.entry(num) {
                Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
            }
        }

        let mut left_count = 0;
        let mut right_count = n;
        let mut result = 0;

        for count in counts.into_values() {
            right_count -= count;
            result += left_count * count * right_count;
            left_count += count;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn unequal_triplets(nums: Vec<i32>) -> i32 {
        Self::unequal_triplets(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
