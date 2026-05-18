pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> bool {
        let k = k.cast_unsigned();
        let n = nums.len() as u32;

        n.is_multiple_of(k) && {
            let groups = n / k;
            let mut counts = HashMap::<i32, u32>::new();

            nums.iter().all(|&num| {
                match counts.entry(num) {
                    Entry::Occupied(occupied_entry) => {
                        let count = occupied_entry.into_mut();

                        if *count == groups {
                            return false;
                        }

                        *count += 1;
                    }
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(1);
                    }
                }

                true
            })
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn partition_array(nums: Vec<i32>, k: i32) -> bool {
        Self::partition_array(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
