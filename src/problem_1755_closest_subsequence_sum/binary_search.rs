pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::slice::Iter;

impl Solution {
    fn get_sums_helper(result: &mut Vec<i32>, mut sum: i32, mut iter: Iter<i32>) {
        while let Some(next) = iter.next() {
            Self::get_sums_helper(result, sum, iter.clone());

            sum += next;
        }

        result.push(sum);
    }

    fn get_sums(nums: &[i32]) -> Box<[i32]> {
        let mut result = Vec::with_capacity(1 << nums.len());

        Self::get_sums_helper(&mut result, 0, nums.iter());

        result.into_boxed_slice()
    }

    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let (left, right) = nums.split_at(nums.len() / 2);
        let left_sums = Self::get_sums(left);
        let mut right_sums = Self::get_sums(right);

        right_sums.sort_unstable();

        let mut result = u32::MAX;

        for left_sum in &*left_sums {
            let target = goal - left_sum;
            let split = right_sums.partition_point(|&x| x < target);
            let diff_1 = right_sums.get(split.wrapping_sub(1)).map_or(-1, |&x| target - x);
            let diff_2 = right_sums.get(split).map_or(-1, |&x| x - target);

            result = result.min(u32::min(diff_1 as _, diff_2 as _));
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        Self::min_abs_difference(nums, goal)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
