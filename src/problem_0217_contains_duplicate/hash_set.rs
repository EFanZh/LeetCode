pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut visited = HashSet::with_capacity(nums.len());

        for num in nums {
            if !visited.insert(num) {
                return true;
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn contains_duplicate(nums: Vec<i32>) -> bool {
        Self::contains_duplicate(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
