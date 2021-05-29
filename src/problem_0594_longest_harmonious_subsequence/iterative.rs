pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::with_capacity(nums.len());

        for num in nums {
            counts.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }

        counts
            .iter()
            .filter_map(|(num, low)| counts.get(&(num + 1)).map(|high| low + high))
            .max()
            .unwrap_or(0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_lhs(nums: Vec<i32>) -> i32 {
        Self::find_lhs(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
