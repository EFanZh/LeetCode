pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::{BuildHasherDefault, DefaultHasher};

impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut frequencies = HashMap::<_, u16, _>::with_hasher(BuildHasherDefault::<DefaultHasher>::default());

        for num in nums {
            if num & 1 == 0 {
                match frequencies.entry(num) {
                    Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(1);
                    }
                }
            }
        }

        let mut result = -1;
        let mut max_frequency = 0;

        for (&num, &frequency) in &frequencies {
            match frequency.cmp(&max_frequency) {
                Ordering::Less => {}
                Ordering::Equal => {
                    if num < result {
                        result = num;
                    }
                }
                Ordering::Greater => {
                    max_frequency = frequency;
                    result = num;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn most_frequent_even(nums: Vec<i32>) -> i32 {
        Self::most_frequent_even(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
