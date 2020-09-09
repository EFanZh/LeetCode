pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = (k + 1) as usize;
        let mut visited = HashSet::with_capacity(nums.len());

        if nums.len() <= k {
            for num in nums {
                if !visited.insert(num) {
                    return true;
                }
            }
        } else {
            let (left, right) = nums.split_at(k);

            for &num in left {
                if !visited.insert(num) {
                    return true;
                }
            }

            for (i, &num) in right.iter().enumerate() {
                visited.remove(&nums[i]);

                if !visited.insert(num) {
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
