pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        nums.sort_unstable_by_key(|num| num.cast_unsigned());

        for [x, y] in nums.as_chunks_mut::<2>().0 {
            mem::swap(x, y);
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_game(nums: Vec<i32>) -> Vec<i32> {
        Self::number_game(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
