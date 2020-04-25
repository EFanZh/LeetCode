pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_index = HashMap::new();

        for (i, num) in nums.into_iter().enumerate() {
            match num_to_index.get(&(target - num)) {
                None => num_to_index.insert(num, i),
                Some(index) => return vec![*index as _, i as _],
            };
        }

        Vec::new()
    }
}

impl super::Solution for Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
