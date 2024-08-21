pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::{Entry, OccupiedEntry};
use std::collections::HashMap;

impl Solution {
    fn unwrap_occupied(entry: Entry<i32, u32>) -> OccupiedEntry<i32, u32> {
        match entry {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(_) => unreachable!(),
        }
    }

    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let mut right_counts = HashMap::new();
        let mut sum = 0;

        for &num in &nums {
            sum += num;

            match right_counts.entry(sum) {
                Entry::Occupied(entry) => *entry.into_mut() += 1,
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }

        let mut result = if sum % 2 == 0 {
            right_counts.get(&(sum / 2)).copied().unwrap_or(0) - u32::from(sum == 0)
        } else {
            0
        };

        let mut left_counts = HashMap::new();
        let mut left_sum = 0;

        for &num in &nums {
            left_sum += num;

            let right_count = Self::unwrap_occupied(right_counts.entry(left_sum));

            if *right_count.get() == 1 {
                right_count.remove();
            } else {
                *right_count.into_mut() -= 1;
            }

            let diff = k - num;
            let new_sum = sum + diff;

            if new_sum % 2 == 0 {
                let left_half = new_sum / 2;
                let right_half = left_half - diff;
                let mut score = u32::from(left_sum == right_half);

                if let Some(&left_count) = left_counts.get(&left_half) {
                    score += left_count;
                }

                if let Some(&right_count) = right_counts.get(&right_half) {
                    score += right_count;
                }

                result = result.max(score - u32::from(new_sum == 0));
            }

            match left_counts.entry(left_sum) {
                Entry::Occupied(entry) => *entry.into_mut() += 1,
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        Self::ways_to_partition(nums, k)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_unwrap_occupied_success() {
        super::Solution::unwrap_occupied(HashMap::from([(2, 3)]).entry(2));
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_unwrap_occupied_failure() {
        super::Solution::unwrap_occupied(HashMap::from([(2, 5)]).entry(5));
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
