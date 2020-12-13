pub struct Solution;

use std::cmp::Ordering;
use std::mem;

impl Solution {
    fn single_max_numbers(nums: &mut [i32], min_length: usize, max_length: usize) -> Box<[Box<[i32]>]> {
        let mut result = Vec::with_capacity(max_length - min_length + 1);

        if max_length == 0 {
            result.push(Box::default());
        } else {
            let n = nums.len();
            let min_remove = n - max_length;
            let max_remove = n - min_length;
            let mut stack_top = 0_usize;
            let mut i = 0_usize;

            loop {
                let removed = i - stack_top;

                if removed.wrapping_sub(result.len()) == min_remove {
                    result.push(nums[..stack_top].iter().chain(&nums[i..]).copied().collect());
                }

                if removed == max_remove {
                    break;
                } else if let Some(&num) = nums.get(i) {
                    let to_remove = min_remove + result.len();
                    let start = i.saturating_sub(to_remove);

                    let insertion_point = start
                        + nums[start..stack_top]
                            .binary_search_by(|&v| if v < num { Ordering::Greater } else { Ordering::Less })
                            .unwrap_err();

                    if insertion_point == n - to_remove {
                        i += 1;
                    } else if insertion_point == stack_top {
                        nums[insertion_point] = num;
                        stack_top = insertion_point + 1;
                        i += 1;
                    } else {
                        stack_top = insertion_point;
                    }
                } else {
                    stack_top -= 1;
                }
            }
        }

        result.into()
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
            (0, k)
        } else if k < nums2.len() {
            (0, nums1.len())
        } else {
            (k - nums2.len(), nums1.len())
        };

        let mut result = vec![0; k];
        let max_nums_1 = Self::single_max_numbers(&mut nums1, range.0, range.1);
        let max_nums_2 = Self::single_max_numbers(&mut nums2, k - range.1, k - range.0);
        let mut merge_buffer = vec![0; k];

        for (max_num_1, max_num_2) in max_nums_1
            .into_vec()
            .into_iter()
            .zip(max_nums_2.into_vec().into_iter().rev())
        {
            Self::merge(&max_num_1, &max_num_2, &mut merge_buffer);

            if merge_buffer > result {
                mem::swap(&mut result, &mut merge_buffer);
            }
        }

        result
    }
}

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
