pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut iter = nums.iter_mut();

        while let (Some(mut left), Some(mut right)) = (iter.next(), iter.next_back()) {
            while *left % 2 == 0 {
                if let Some(new_left) = iter.next() {
                    left = new_left;
                } else {
                    return nums;
                }
            }

            while *right % 2 != 0 {
                if let Some(new_right) = iter.next_back() {
                    right = new_right;
                } else {
                    return nums;
                }
            }

            mem::swap(left, right);
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_array_by_parity(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
