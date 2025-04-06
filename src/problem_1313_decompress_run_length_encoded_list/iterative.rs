pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let iter = nums.iter().copied();
        let mut iter_2 = iter.clone();
        let mut result = Vec::with_capacity(nums.len());

        iter_2.next();

        for (count, num) in iter.zip(iter_2).step_by(2) {
            result.extend(iter::repeat_n(num, count as _));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        Self::decompress_rl_elist(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
