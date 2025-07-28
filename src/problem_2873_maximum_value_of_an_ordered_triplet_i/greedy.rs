pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(|num| num as u32).collect::<Vec<_>>();
        let n = nums.len();

        assert!(matches!(n, 2..=100));

        let mut left = [0; 98];
        let mut max = 0;

        left.iter_mut().zip(&nums[..n - 2]).for_each(|(target, &num)| {
            max = max.max(num);
            *target = max;
        });

        let mut right = [0; 98];
        let mut max = 0;

        right.iter_mut().zip(&nums[2..]).rev().for_each(|(target, &num)| {
            max = max.max(num);
            *target = max;
        });

        nums[1..]
            .iter()
            .zip(iter::zip(&left, &right))
            .fold(0, |max, (&middle, (&left_max, &right_max))| {
                max.max(i64::from(left_max.wrapping_sub(middle) as i32) * i64::from(right_max))
            })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        Self::maximum_triplet_value(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
