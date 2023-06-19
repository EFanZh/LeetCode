pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::mem;

enum State {
    BothUnknown,
    HasLeft,
    HasRight,
    HasBoth,
}

impl Solution {
    fn make_iter(nums: &[i32]) -> impl Iterator<Item = u32> + '_ {
        nums.iter().map(|&num| num as _)
    }

    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut left_iter = Self::make_iter(&nums1);
        let mut left = 0;
        let mut left_sum = 0;
        let mut right_iter = Self::make_iter(&nums2);
        let mut right = 0;
        let mut right_sum = 0;
        let mut result = 0;
        let mut state = State::BothUnknown;

        loop {
            state = match state {
                State::BothUnknown => {
                    left_sum = 0;
                    right_sum = 0;

                    if let Some(new_left) = left_iter.next() {
                        left = new_left;
                        left_sum += u64::from(new_left);

                        State::HasLeft
                    } else {
                        break;
                    }
                }
                State::HasLeft => {
                    if let Some(new_right) = right_iter.next() {
                        right = new_right;
                        right_sum += u64::from(new_right);

                        State::HasBoth
                    } else {
                        mem::swap(&mut left_sum, &mut right_sum);
                        right_iter = left_iter;

                        break;
                    }
                }
                State::HasRight => {
                    if let Some(new_left) = left_iter.next() {
                        left = new_left;
                        left_sum += u64::from(new_left);

                        State::HasBoth
                    } else {
                        break;
                    }
                }
                State::HasBoth => match left.cmp(&right) {
                    Ordering::Less => State::HasRight,
                    Ordering::Equal => {
                        result += left_sum.max(right_sum);

                        State::BothUnknown
                    }
                    Ordering::Greater => State::HasLeft,
                },
            };
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
