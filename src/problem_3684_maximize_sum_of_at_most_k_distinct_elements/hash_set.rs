pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn max_k_distinct(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums;
        let k = k.cast_unsigned() as usize;
        let set = nums.iter().copied().collect::<HashSet<_>>();

        nums.clear();
        nums.extend(set);

        let compare = |lhs: &_, rhs: &_| i32::cmp(rhs, lhs);

        if k < nums.len() {
            nums.select_nth_unstable_by(k, compare);
            nums.truncate(k);
        }

        nums.sort_unstable_by(compare);

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_k_distinct(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::max_k_distinct(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
