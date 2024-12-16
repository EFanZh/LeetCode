pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut counts = HashMap::new();
        let mut good_pairs = 0;

        (0..).zip(nums).for_each(|(i, num)| {
            let key = num - i;

            match counts.entry(key) {
                Entry::Occupied(occupied_entry) => {
                    let count = occupied_entry.into_mut();

                    good_pairs += *count;

                    *count += 1;
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(1);
                }
            }
        });

        let n = n as u64;

        (n * (n - 1) / 2 - good_pairs) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        Self::count_bad_pairs(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
