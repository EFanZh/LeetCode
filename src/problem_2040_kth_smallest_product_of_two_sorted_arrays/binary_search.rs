pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn check(
        (left_negative, left_zero, left_positive): (&[i32], &[i32], &[i32]),
        (right_negative, right_zero, right_positive): (&[i32], &[i32], &[i32]),
        k: u64,
        middle: i64,
    ) -> bool {
        let mut count = 0;

        if middle < 0 {
            let mut iter = right_positive.iter();

            'outer: for &lhs in left_negative {
                loop {
                    if let Some(&first) = iter.as_slice().first() {
                        if i64::from(lhs) * i64::from(first) <= middle {
                            count += iter.len() as u64;

                            break;
                        }

                        iter.next();
                    } else {
                        break 'outer;
                    }
                }
            }

            iter = right_negative.iter();

            'outer: for &lhs in left_positive.iter().rev() {
                loop {
                    if let Some(&last) = iter.as_slice().last() {
                        if i64::from(lhs) * i64::from(last) <= middle {
                            count += iter.len() as u64;

                            break;
                        }

                        iter.next_back();
                    } else {
                        break 'outer;
                    }
                }
            }
        } else {
            count += left_negative.len() as u64 * (right_zero.len() + right_positive.len()) as u64;
            count += left_zero.len() as u64 * (right_negative.len() + right_zero.len() + right_positive.len()) as u64;
            count += left_positive.len() as u64 * (right_negative.len() + right_zero.len()) as u64;

            let mut iter = right_negative.iter();

            'outer: for &lhs in left_negative.iter().rev() {
                loop {
                    if let Some(&first) = iter.as_slice().first() {
                        if i64::from(lhs) * i64::from(first) <= middle {
                            count += iter.len() as u64;

                            break;
                        }

                        iter.next();
                    } else {
                        break 'outer;
                    }
                }
            }

            iter = right_positive.iter();

            'outer: for &lhs in left_positive {
                loop {
                    if let Some(&last) = iter.as_slice().last() {
                        if i64::from(lhs) * i64::from(last) <= middle {
                            count += iter.len() as u64;

                            break;
                        }

                        iter.next_back();
                    } else {
                        break 'outer;
                    }
                }
            }
        }

        count < k
    }

    fn min_max(mut a: i64, mut b: i64, mut c: i64, mut d: i64) -> (i64, i64) {
        fn sort_2(a: &mut i64, b: &mut i64) {
            if b < a {
                mem::swap(a, b);
            }
        }

        sort_2(&mut a, &mut b);
        sort_2(&mut c, &mut d);

        (a.min(c), b.max(d))
    }

    fn split_at_partition_point(nums: &[i32], f: impl FnMut(&i32) -> bool) -> (&[i32], &[i32]) {
        let i = nums.partition_point(f);

        nums.split_at(i)
    }

    fn split_3(nums: &[i32]) -> (&[i32], &[i32], &[i32]) {
        let (negative, rest) = Self::split_at_partition_point(nums, |&x| x < 0);
        let (zero, positive) = Self::split_at_partition_point(rest, |&x| x == 0);

        (negative, zero, positive)
    }

    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        let k = k as u64;
        let min_1 = i64::from(*nums1.first().unwrap());
        let max_1 = i64::from(*nums1.last().unwrap());
        let min_2 = i64::from(*nums2.first().unwrap());
        let max_2 = i64::from(*nums2.last().unwrap());
        let (mut left, mut right) = Self::min_max(min_1 * min_2, min_1 * max_2, max_1 * min_2, max_1 * max_2);
        let split_1 = Self::split_3(&nums1);
        let split_2 = Self::split_3(&nums2);

        while left < right {
            let middle = (left + right).div_euclid(2);

            if Self::check(split_1, split_2, k, middle) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        Self::kth_smallest_product(nums1, nums2, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
