pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{hash_map, HashMap, HashSet};

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            let mut counts = HashMap::<_, bool>::with_capacity(nums.len());
            let mut result = 0;

            for num in nums {
                match counts.entry(num) {
                    hash_map::Entry::Occupied(entry) => {
                        if !*entry.get() {
                            result += 1;

                            *entry.into_mut() = true;
                        }
                    }
                    hash_map::Entry::Vacant(entry) => {
                        entry.insert(false);
                    }
                }
            }

            result
        } else {
            let nums = nums.into_iter().collect::<HashSet<_>>();

            nums.iter().filter(|&num| nums.contains(&(num + k))).count() as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        Self::find_pairs(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
