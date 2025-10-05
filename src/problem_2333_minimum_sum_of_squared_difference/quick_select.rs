pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

struct Random<const N: usize>(usize);

impl<const N: usize> Random<N> {
    fn next(&mut self, seed: usize) -> usize {
        self.0 = self.0.wrapping_add(seed).wrapping_mul(N);

        self.0
    }
}

impl Solution {
    fn three_way_partition(nums: &mut [u32], key: u32) -> (usize, usize) {
        let n = nums.len();
        let mut less_count = 0;
        let mut iter_start = 0;
        let mut iter_end = n;

        'outer: while iter_start < iter_end {
            let left_num = nums[iter_start];

            match left_num.cmp(&key) {
                Ordering::Less => {
                    if less_count < iter_start {
                        nums[less_count] = left_num;
                        nums[iter_start] = key;
                    }

                    less_count += 1;
                }
                Ordering::Equal => {}
                Ordering::Greater => loop {
                    iter_end -= 1;

                    if iter_start < iter_end {
                        let right_num = nums[iter_end];

                        if right_num <= key {
                            nums[iter_end] = left_num;

                            if right_num < key {
                                nums[less_count] = right_num;

                                if less_count < iter_start {
                                    nums[iter_start] = key;
                                }

                                less_count += 1;
                            } else {
                                nums[iter_start] = key;
                            }

                            break;
                        }
                    } else {
                        break 'outer;
                    }
                },
            }

            iter_start += 1;
        }

        (less_count, iter_start)
    }

    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut nums1 = nums1;

        nums1
            .iter_mut()
            .zip(nums2)
            .for_each(|(target, source)| *target = target.abs_diff(source) as _);

        let mut nums = nums1.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let nums = &mut *nums;
        let n = nums.len();
        let k = u64::from((k1 + k2) as u32);
        let mut random = Random::<0x_9E37_79B9>(0);
        let mut left = 0;
        let mut right = n;
        let mut right_sum = 0;

        while left < right {
            let window = &mut nums[left..right];
            let window_len = window.len();
            let middle_num = window[random.next(window_len) % window_len];
            let (mut middle_start, mut middle_end) = Self::three_way_partition(window, middle_num);

            middle_start += left;
            middle_end += left;

            let (upper_bound, new_left, new_right) = if middle_end - middle_start < 2 {
                (
                    if middle_start == left {
                        nums.get(middle_start.wrapping_sub(1)).copied().unwrap_or(0)
                    } else {
                        nums[left..middle_start].iter().copied().max().unwrap()
                    },
                    middle_start + 1,
                    middle_start,
                )
            } else {
                (middle_num, middle_end, middle_start + 1)
            };

            let candidate_right_sum = nums[middle_end..right].iter().fold(
                u64::from(middle_num) * (middle_end - new_right) as u64 + right_sum,
                |sum, &x| sum + u64::from(x),
            );

            let extra = candidate_right_sum - u64::from(upper_bound) * (n - new_right) as u64;

            if k < extra {
                left = new_left;
            } else {
                right_sum = candidate_right_sum;
                right = new_right;
            }
        }

        nums.get(left.wrapping_sub(1)).map_or(0, |&upper_bound| {
            let upper_bound = u64::from(upper_bound);
            let mut right_len = (n - left) as u64;
            let remaining = k - (right_sum - upper_bound * right_len);

            right_len += 1;

            let quotient = remaining / right_len;
            let remainder = remaining % right_len;

            nums[..left - 1].iter().fold(
                (upper_bound - quotient - 1).pow(2) * remainder
                    + (upper_bound - quotient).pow(2) * (right_len - remainder),
                |sum, &x| sum + u64::from(x).pow(2),
            )
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        Self::min_sum_square_diff(nums1, nums2, k1, k2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
