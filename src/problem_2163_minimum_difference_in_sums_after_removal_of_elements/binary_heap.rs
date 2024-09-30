pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        let n = nums.len() / 3;
        let middle_original = nums[n..n * 2].to_vec().into_boxed_slice();
        let mut rest = nums[n..].iter().copied().zip(n as u32..).collect::<Vec<_>>();

        rest.select_nth_unstable(n);

        let mut right_sum = 0;

        let mut right_min = rest.drain(n..).fold((i32::MAX, u32::MAX), |min, x| {
            right_sum += i64::from(x.0);

            min.min(x)
        });

        let mut middle = BinaryHeap::from(rest);

        nums.truncate(n);

        let mut left = BinaryHeap::from(nums);
        let mut diff = left.iter().fold(0_i64, |sum, &x| sum + i64::from(x)) - right_sum;
        let mut min_diff = diff;

        middle_original.iter().copied().zip(n as u32..).for_each(|item| {
            let mut left_top = left.peek_mut().unwrap();

            if item.0 < *left_top {
                diff += i64::from(item.0 - *left_top);
                *left_top = item.0;
            }

            drop(left_top);

            if item >= right_min {
                let mut middle_top;

                loop {
                    middle_top = middle.pop().unwrap();

                    if middle_top.1 > item.1 {
                        break;
                    }
                }

                diff += i64::from(item.0 - middle_top.0);
                right_min = right_min.min(middle_top);
            }

            min_diff = min_diff.min(diff);
        });

        min_diff
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_difference(nums: Vec<i32>) -> i64 {
        Self::minimum_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
