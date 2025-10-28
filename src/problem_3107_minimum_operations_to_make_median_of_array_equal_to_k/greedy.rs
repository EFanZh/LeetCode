pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn min_operations_to_make_median_k(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k.cast_unsigned();

        nums.sort_unstable();

        let middle = nums.len() / 2;

        match nums[middle].cmp(&k) {
            Ordering::Less => nums[middle..]
                .iter()
                .map_while(|&num| k.checked_sub(num).map(i64::from))
                .sum(),
            Ordering::Equal => 0,
            Ordering::Greater => nums[..=middle]
                .iter()
                .rev()
                .map_while(|&num| num.checked_sub(k).map(i64::from))
                .sum(),
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations_to_make_median_k(nums: Vec<i32>, k: i32) -> i64 {
        Self::min_operations_to_make_median_k(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
