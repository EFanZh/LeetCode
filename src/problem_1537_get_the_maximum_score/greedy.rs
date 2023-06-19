pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::mem;

impl Solution {
    fn make_iter(nums: &[i32]) -> impl Iterator<Item = u32> + '_ {
        nums.iter().map(|&num| num as _)
    }

    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut left_iter = Self::make_iter(&nums1);
        let mut right_iter = Self::make_iter(&nums2);
        let mut left_sum: u64;
        let mut right_sum: u64;
        let mut result = 0_u64;

        // Both unknown.

        'outer: loop {
            left_sum = 0;
            right_sum = 0;

            if let Some(mut left) = left_iter.next() {
                left_sum += u64::from(left);

                // Has left.

                for right in right_iter.by_ref() {
                    right_sum += u64::from(right);

                    // Has left and right.

                    loop {
                        match left.cmp(&right) {
                            Ordering::Less => {
                                // Has right.

                                if let Some(new_left) = left_iter.next() {
                                    left = new_left;
                                    left_sum += u64::from(new_left);
                                } else {
                                    break 'outer;
                                }
                            }
                            Ordering::Equal => {
                                result += left_sum.max(right_sum);

                                continue 'outer;
                            }
                            Ordering::Greater => break,
                        }
                    }
                }

                mem::swap(&mut left_sum, &mut right_sum);
                right_iter = left_iter;
            }

            break;
        }

        for num in right_iter {
            right_sum += u64::from(num);
        }

        ((result + left_sum.max(right_sum)) % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        Self::max_sum(nums1, nums2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
