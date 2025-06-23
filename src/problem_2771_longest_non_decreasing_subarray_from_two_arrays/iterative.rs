pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::{iter, mem};

impl Solution {
    pub fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut prev_1 = 0;
        let mut length_1 = 0;
        let mut prev_2 = 0;
        let mut length_2 = 0;
        let mut result = 0;

        iter::zip(nums1, nums2).for_each(|(num1, num2)| {
            let (mut x, mut y) = (num1 as u32, num2 as u32);

            if y < x {
                mem::swap(&mut x, &mut y);
            }

            let new_length_1 = if x < prev_1 {
                1
            } else {
                (if x < prev_2 { length_1 } else { length_2 }) + 1
            };

            let new_length_2 = if y < prev_1 {
                1
            } else {
                (if y < prev_2 { length_1 } else { length_2 }) + 1
            };

            prev_1 = x;
            length_1 = new_length_1;
            prev_2 = y;
            length_2 = new_length_2;
            result = u32::max(result, length_2);
        });

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::max_non_decreasing_length(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
