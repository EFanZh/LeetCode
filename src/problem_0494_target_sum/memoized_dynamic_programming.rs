pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn helper(nums: &[i32], target: i32, cache: &mut HashMap<(usize, i32), i32>) -> i32 {
        let key = (nums.len(), target);

        if let Some(&result) = cache.get(&key) {
            result
        } else {
            let result = if let Some((&first, rest)) = nums.split_first() {
                if first <= target {
                    Self::helper(rest, target - first, cache) + Self::helper(rest, target, cache)
                } else {
                    Self::helper(rest, target, cache)
                }
            } else if target == 0 {
                1
            } else {
                0
            };

            cache.insert(key, result);

            result
        }
    }

    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let mut nums = nums;
        let sum = nums.iter().sum::<i32>();

        if s >= -sum && s <= sum {
            let target = s + sum;

            for num in &mut nums {
                *num *= 2;
            }

            Self::helper(&nums, target, &mut HashMap::new())
        } else {
            0
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        Self::find_target_sum_ways(nums, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
