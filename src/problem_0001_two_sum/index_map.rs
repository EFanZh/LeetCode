pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (last, nums) = nums.split_last().unwrap();
        let mut indices = HashMap::new();

        for (i, &num) in (0..).zip(nums) {
            match indices.get(&(target - num)) {
                None => indices.insert(num, i),
                Some(&index) => return vec![index, i],
            };
        }

        vec![indices[&(target - last)], nums.len() as _]
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
