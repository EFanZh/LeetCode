pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    fn calculate_diff(x: u32) -> u32 {
        let mut y = x;
        let mut rev = 0;

        while y != 0 {
            rev = rev * 10 + y % 10;
            y /= 10;
        }

        x.wrapping_sub(rev)
    }

    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut diffs = HashMap::new();
        let mut result = 0_u64;

        for num in nums {
            match diffs.entry(Self::calculate_diff(num as _)) {
                Entry::Occupied(entry) => {
                    result += *entry.get();
                    *entry.into_mut() += 1;
                }
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }

        (result % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        Self::count_nice_pairs(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
