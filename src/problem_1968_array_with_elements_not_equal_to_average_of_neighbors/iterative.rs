pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut iter = nums.iter_mut();
        let prev_1 = iter.next().unwrap();
        let mut prev_2 = iter.next().unwrap();
        let mut is_decreasing = prev_2 < prev_1;

        for value in iter {
            if is_decreasing == (value < prev_2) {
                mem::swap(prev_2, value);
            }

            prev_2 = value;
            is_decreasing = !is_decreasing;
        }

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        Self::rearrange_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
