pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    fn two_sum_closest_head(head: i32, rest: &[i32], target: i32, result: &mut i32) {
        if let Some((&tail, body)) = rest.split_last() {
            Self::two_sum_closest_dispatch(head, tail, body, target, result);
        }
    }

    fn two_sum_closest_tail(tail: i32, rest: &[i32], target: i32, result: &mut i32) {
        if let Some((&head, body)) = rest.split_first() {
            Self::two_sum_closest_dispatch(head, tail, body, target, result);
        }
    }

    fn two_sum_closest_dispatch(head: i32, tail: i32, body: &[i32], target: i32, result: &mut i32) {
        let sum = head + tail;

        if (target - sum).abs() < (target - *result).abs() {
            *result = sum;
        }

        match sum.cmp(&target) {
            Ordering::Less => Self::two_sum_closest_tail(tail, body, target, result),
            Ordering::Equal => {}
            Ordering::Greater => Self::two_sum_closest_head(head, body, target, result),
        }
    }

    fn two_sum_closest(head: i32, tail: i32, body: &[i32], target: i32) -> i32 {
        let mut sum = head + tail;

        match sum.cmp(&target) {
            Ordering::Less => Self::two_sum_closest_tail(tail, body, target, &mut sum),
            Ordering::Equal => {}
            Ordering::Greater => Self::two_sum_closest_head(head, body, target, &mut sum),
        }

        sum
    }

    fn three_sum_closest_helper(first: i32, last: i32, body: &[i32], target: i32, result: &mut i32) {
        if let Some((&second, body)) = body.split_first() {
            let three_sum = first + Self::two_sum_closest(second, last, body, target - first);

            if (target - three_sum).abs() < (*result - target).abs() {
                *result = three_sum;
            }

            Self::three_sum_closest_helper(second, last, body, target, result);
        }
    }

    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        if let Some((&mut first, rest)) = nums.split_first_mut() {
            rest.sort_unstable();

            if let Some((&second, rest)) = rest.split_first() {
                if let Some((&last, rest)) = rest.split_last() {
                    let mut three_sum = first + Self::two_sum_closest(second, last, rest, target - first);

                    Self::three_sum_closest_helper(second, last, rest, target, &mut three_sum);

                    return three_sum;
                }
            }
        }

        unreachable!();
    }
}

impl super::Solution for Solution {
    fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        Self::three_sum_closest(nums, target)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
