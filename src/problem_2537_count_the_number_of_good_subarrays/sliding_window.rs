pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let k = k.cast_unsigned();
        let mut start = 0;
        let mut counts = HashMap::<_, u16>::new();
        let mut pairs = 0;
        let mut result = 0;

        for &num in &nums {
            match counts.entry(num) {
                Entry::Occupied(occupied_entry) => {
                    pairs += u32::from(*occupied_entry.get());
                    *occupied_entry.into_mut() += 1;
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
            }

            while pairs >= k {
                let old = nums[start];
                let count = counts.get_mut(&old).unwrap();

                start += 1;
                *count -= 1;
                pairs -= u32::from(*count);
            }

            result += start as u64;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        Self::count_good(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
