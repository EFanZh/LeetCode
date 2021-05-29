pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::mem;

impl Solution {
    fn single_max_numbers(nums: &mut [i32], min_length: usize, max_length: usize) -> Box<[Box<[i32]>]> {
        let expected_items = max_length - min_length + 1;
        let mut result = Vec::with_capacity(expected_items);

        if max_length == 0 {
            result.push(Box::default());
        } else {
            let k = nums.len() - max_length;

            let (mut stack_base, mut stack_top, mut i) =
                if let Some(stack_base) = nums[..k].iter().zip(&nums[1..]).position(|(lhs, rhs)| lhs >= rhs) {
                    let stack_top_end = stack_base + max_length;

                    let mut stack_top = nums[stack_base + 1..stack_top_end]
                        .iter()
                        .zip(&nums[stack_base + 2..])
                        .position(|(lhs, rhs)| lhs < rhs)
                        .map_or(stack_top_end, |p| stack_base + 1 + p);

                    let mut i = stack_top + 1;
                    let to_remove = k - stack_base;

                    loop {
                        if let Some(&num) = nums.get(i) {
                            let start = stack_base + (i - stack_base).saturating_sub(to_remove);

                            let insertion_point = start
                                + nums[start..stack_top]
                                    .binary_search_by(|&v| if v >= num { Ordering::Less } else { Ordering::Greater })
                                    .unwrap_err();

                            if i - insertion_point == to_remove {
                                stack_top = insertion_point;
                                result.push(nums[stack_base..stack_top].iter().chain(&nums[i..]).copied().collect());

                                break;
                            } else if insertion_point != stack_top_end {
                                nums[insertion_point] = num;
                                stack_top = insertion_point + 1;
                            }

                            i += 1;
                        } else {
                            result.push(nums[stack_base..stack_top].into());

                            break;
                        }
                    }

                    (stack_base, stack_top, i)
                } else {
                    result.push(nums[k..].into());

                    (k, k + 1, k + 1)
                };

            if result.len() != expected_items {
                if let Some(mut num) = nums.get(i).copied() {
                    loop {
                        if num > nums[stack_top - 1] {
                            if stack_top - stack_base == 1 {
                                result.push(nums[i..].into());

                                if result.len() == expected_items {
                                    break;
                                }

                                stack_base = i;
                                stack_top = i + 1;
                                i += 1;

                                if let Some(&next_num) = nums.get(i) {
                                    num = next_num;
                                } else {
                                    break;
                                }
                            } else {
                                stack_top -= 1;
                                result.push(nums[stack_base..stack_top].iter().chain(&nums[i..]).copied().collect());

                                if result.len() == expected_items {
                                    break;
                                }
                            }
                        } else {
                            nums[stack_top] = num;
                            stack_top += 1;
                            i += 1;

                            if let Some(&next_num) = nums.get(i) {
                                num = next_num;
                            } else {
                                break;
                            }
                        }
                    }
                }

                for top in ((stack_base + min_length)..stack_top).rev() {
                    result.push(nums[stack_base..top].into());
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
        } else {
            (k.saturating_sub(nums2.len()), nums1.len())
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
