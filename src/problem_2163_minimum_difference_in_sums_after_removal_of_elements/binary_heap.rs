pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        let n = nums.len() / 3;
        let middle_original = nums[n..n * 2].to_vec().into_boxed_slice();
        let mut rest = nums.drain(n..).zip(0_u32..).collect::<Box<_>>();

        rest.select_nth_unstable(n);

        let (middle, right) = rest.split_at_mut(n);
        let mut left = BinaryHeap::from(nums);

        let mut diff = left.iter().fold(0_i64, |sum, &x| sum + i64::from(x))
            - right.iter().fold(0, |sum, &(x, _)| sum + i64::from(x));

        let mut min_diff = diff;

        middle.sort_unstable();

        let mut middle_iter = middle.iter();

        middle_original.iter().copied().zip(0_u32..).for_each(|item| {
            let mut left_top = left.peek_mut().unwrap();

            if item.0 < *left_top {
                diff += i64::from(item.0 - *left_top);
                *left_top = item.0;
            }

            drop(left_top);

            let mut middle_max = *middle_iter.as_slice().last().unwrap();

            if item > middle_max {
                loop {
                    middle_iter.next_back();

                    if middle_max.1 <= item.1 {
                        middle_max = *middle_iter.as_slice().last().unwrap();
                    } else {
                        break;
                    }
                }

                diff += i64::from(item.0 - middle_max.0);
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
