pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let nums = nums.as_slice();
        let n = nums.len();

        assert!(n > 3);

        let mut sums = HashMap::new();

        sums.insert(nums[0] + nums[1], 1);

        let mut result = 0;

        for c in 2..n - 1 {
            let num_c = nums[c];

            for &num_d in &nums[c + 1..] {
                result += sums.get(&(num_d - num_c)).copied().unwrap_or(0);
            }

            for &num_a in &nums[..c] {
                match sums.entry(num_a + num_c) {
                    Entry::Occupied(entry) => *entry.into_mut() += 1,
                    Entry::Vacant(entry) => {
                        entry.insert(1);
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_quadruplets(nums: Vec<i32>) -> i32 {
        Self::count_quadruplets(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
