pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut map = HashMap::new();

        for &num in &nums {
            sum += num;

            match map.entry(num) {
                Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() = true,
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(false);
                }
            }
        }

        let mut result = i32::MIN;

        for &num in &nums {
            if num > result {
                let rest = sum - num;

                if rest & 1 == 0 {
                    let half = rest / 2;

                    if map.get(&half).is_some_and(|&state| state || half != num) {
                        result = num;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        Self::get_largest_outlier(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
