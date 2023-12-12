pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut counts = HashMap::new();

        for num in nums {
            counts.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut result = 0;

        for (num, count) in &counts {
            result += count * counts.get(&(num + k)).copied().unwrap_or(0);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        Self::count_k_difference(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
