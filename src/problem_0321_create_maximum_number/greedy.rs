pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::mem;

impl Solution {
    fn single_max_number(num: &[i32], result: &mut [i32]) {
        if !result.is_empty() {
            let result_length = result.len();
            let k = num.len() - result_length;
            let mut stack_length = 0;
            let mut i = 0;

            while i - stack_length != k {
                let start = i.saturating_sub(k);
                let digit = num[i];

                let insertion_point = start
                    + result[start..stack_length]
                        .binary_search_by(|&v| if v < digit { Ordering::Greater } else { Ordering::Less })
                        .unwrap_err();

                if insertion_point != result_length {
                    result[insertion_point] = digit;
                    stack_length = insertion_point + 1;
                }

                i += 1;
            }

            result[stack_length..].copy_from_slice(&num[i..]);
        }
    }

    fn merge(nums_1: &[i32], nums_2: &[i32], result: &mut [i32]) {
        let mut i = 0;

        if let Some((&left, mut rest_left)) = nums_1.split_first() {
            if let Some((&right, mut rest_right)) = nums_2.split_first() {
                let mut left = left;
                let mut right = right;

                loop {
                    if (left, rest_left) < (right, rest_right) {
                        result[i] = right;
                        i += 1;

                        if let Some((&next_right, next_rest_right)) = rest_right.split_first() {
                            right = next_right;
                            rest_right = next_rest_right;
                        } else {
                            result[i..].copy_from_slice(&nums_1[nums_1.len() - rest_left.len() - 1..]);

                            break;
                        }
                    } else {
                        result[i] = left;
                        i += 1;

                        if let Some((&next_left, next_rest_left)) = rest_left.split_first() {
                            left = next_left;
                            rest_left = next_rest_left;
                        } else {
                            result[i..].copy_from_slice(&nums_2[nums_2.len() - rest_right.len() - 1..]);

                            break;
                        }
                    }
                }
            } else {
                result.copy_from_slice(nums_1);
            }
        } else {
            result.copy_from_slice(nums_2);
        }
    }

    pub fn max_number(mut nums1: Vec<i32>, mut nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        if nums2.len() < nums1.len() {
            mem::swap(&mut nums1, &mut nums2);
        }

        let range = if k < nums1.len() {
            0..=k
        } else if k < nums2.len() {
            0..=nums1.len()
        } else {
            k - nums2.len()..=nums1.len()
        };

        let mut result = vec![0; k];
        let mut max_buffer = vec![0; k];
        let mut merge_buffer = vec![0; k];

        for i in range {
            let (max_1, max_2) = max_buffer.split_at_mut(i);

            Self::single_max_number(&nums1, max_1);
            Self::single_max_number(&nums2, max_2);
            Self::merge(max_1, max_2, &mut merge_buffer);

            if merge_buffer > result {
                mem::swap(&mut result, &mut merge_buffer);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        Self::max_number(nums1, nums2, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
