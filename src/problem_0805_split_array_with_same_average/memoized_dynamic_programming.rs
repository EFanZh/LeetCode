pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    fn helper(nums: &[i32], sum: u32, count: u16, target: u32, cache: &mut HashSet<u64>) -> bool {
        // Invariant: `nums.iter().sum() == sum`.
        // Invariant: `0 <= count <= nums.len()`.
        // Invariant: `0 <= target <= sum`.

        if count == 0 {
            target == 0
        } else if count == nums.len() as u16 {
            target == sum
        } else {
            let key = ((nums.len() as u64) << 48) | (u64::from(count) << 32) | u64::from(target);

            if cache.contains(&key) {
                false
            } else {
                let (&first, nums) = nums.split_first().unwrap();
                let first = first as u32;
                let sum = sum - first;

                let result = first <= target
                    && (Self::helper(nums, sum, count - 1, target - first, cache)
                        || (target <= sum && Self::helper(nums, sum, count, target, cache)));

                if !result {
                    cache.insert(key);
                }

                result
            }
        }
    }

    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let mut nums = nums;

        nums.sort_unstable();

        let n = nums.len() as u32;
        let sum = nums.iter().sum::<i32>() as u32;
        let mut cache = HashSet::new();

        for count in 1..=n / 2 {
            let denominator = sum * count;

            if denominator % n == 0 && Self::helper(&nums, sum, count as _, denominator / n, &mut cache) {
                return true;
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn split_array_same_average(nums: Vec<i32>) -> bool {
        Self::split_array_same_average(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
