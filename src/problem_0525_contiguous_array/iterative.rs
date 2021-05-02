pub struct Solution;

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut indices = HashMap::with_capacity(nums.len());
        let mut result = 0;

        indices.insert(0, -1);

        for (i, num) in (0..).zip(nums) {
            sum += if num == 0 { -1 } else { 1 };

            match indices.entry(sum) {
                Entry::Occupied(entry) => result = result.max(i - entry.get()),
                Entry::Vacant(entry) => {
                    entry.insert(i);
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_max_length(nums: Vec<i32>) -> i32 {
        Self::find_max_length(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
