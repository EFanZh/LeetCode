pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut indices = HashMap::with_capacity(nums.len());

        for (i, num) in (0..).zip(nums.into_iter()) {
            if let Some(old_index) = indices.insert(num, i) {
                if i - old_index <= k {
                    return true;
                }
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        Self::contains_nearby_duplicate(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
