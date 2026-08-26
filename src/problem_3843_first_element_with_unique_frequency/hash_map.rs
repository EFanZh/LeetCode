pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn first_unique_freq(nums: Vec<i32>) -> i32 {
        let mut frequencies = HashMap::new();

        for &num in &nums {
            match frequencies.entry(num) {
                Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(0);
                }
            }
        }

        let mut frequency_frequencies = HashMap::new();

        for &frequency in frequencies.values() {
            match frequency_frequencies.entry(frequency) {
                Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() = true,
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(false);
                }
            }
        }

        nums.iter()
            .copied()
            .find(|num| !frequency_frequencies[&frequencies[num]])
            .unwrap_or(-1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn first_unique_freq(nums: Vec<i32>) -> i32 {
        Self::first_unique_freq(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
