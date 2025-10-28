pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::iter;

impl Solution {
    pub fn min_operations_to_make_median_k(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let k = k.cast_unsigned();

        nums.sort_unstable();

        let half = nums.len() / 2;
        let (left, &mut middle, right) = nums.select_nth_unstable(half);

        match middle.cmp(&k) {
            Ordering::Less => iter::once(middle)
                .chain(right.iter().copied())
                .map(|num| i64::from(k.saturating_sub(num)))
                .sum(),
            Ordering::Equal => 0,
            Ordering::Greater => iter::once(middle)
                .chain(left.iter().copied())
                .map(|num| i64::from(num.saturating_sub(k)))
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
