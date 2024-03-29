pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

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

    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        let (&mut first, rest) = nums.split_first_mut().unwrap();

        rest.sort_unstable();

        let (&second, rest) = rest.split_first().unwrap();
        let (&last, rest) = rest.split_last().unwrap();
        let mut three_sum = first + Self::two_sum_closest(second, last, rest, target - first);

        Self::three_sum_closest_helper(second, last, rest, target, &mut three_sum);

        three_sum
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        Self::three_sum_closest(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
