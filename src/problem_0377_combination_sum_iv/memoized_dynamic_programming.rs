pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn helper(nums: &[i32], target: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if target == 0 {
            1
        } else if let Some(&result) = cache.get(&target) {
            result
        } else {
            let mut result = 0;

            for num in nums.iter().take_while(|&&x| x <= target) {
                result += Self::helper(nums, target - num, cache);
            }

            cache.insert(target, result);

            result
        }
    }

    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        Self::helper(&nums, target, &mut HashMap::new())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        Self::combination_sum4(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
