pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = HashMap::new();
        let mut result = 0;

        for num in nums {
            if let Entry::Occupied(entry) = counts.entry(k - num) {
                if *entry.get() == 1 {
                    entry.remove();
                } else {
                    *entry.into_mut() -= 1;
                }

                result += 1;
            } else {
                counts.entry(num).and_modify(|count| *count += 1).or_insert(1);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        Self::max_operations(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
