pub struct Solution;

use std::cmp::Ordering;
use std::collections::HashSet;

impl Solution {
    fn helper(size: i32, nums: &[i32], cache: &mut HashSet<(u8, u8)>) -> bool {
        if cache.insert((size as _, nums.len() as _)) {
            if let Some((first, rest)) = nums.split_first() {
                if match first.cmp(&size) {
                    Ordering::Less => Self::helper(size - first, rest, cache) || Self::helper(size, rest, cache),
                    Ordering::Equal => true,
                    Ordering::Greater => Self::helper(size, rest, cache),
                } {
                    return true;
                }
            }
        }

        false
    }

    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();

        sum % 2 == 0 && Self::helper(sum / 2, &nums, &mut HashSet::new())
    }
}

impl super::Solution for Solution {
    fn can_partition(nums: Vec<i32>) -> bool {
        Self::can_partition(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
