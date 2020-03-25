pub struct Solution {}

use std::cmp::Ordering;

impl Solution {
    fn two_sum_head(first: (usize, i32), rest: &[(usize, i32)], target: i32) -> Option<[usize; 2]> {
        rest.split_last()
            .and_then(|(last, body)| Self::dispatch(first, *last, body, target))
    }

    fn two_sum_tail(last: (usize, i32), rest: &[(usize, i32)], target: i32) -> Option<[usize; 2]> {
        rest.split_first()
            .and_then(|(first, body)| Self::dispatch(*first, last, body, target))
    }

    fn dispatch(first: (usize, i32), last: (usize, i32), body: &[(usize, i32)], target: i32) -> Option<[usize; 2]> {
        match (first.1 + last.1).cmp(&target) {
            Ordering::Less => Self::two_sum_tail(last, body, target),
            Ordering::Equal => Some([first.0, last.0]),
            Ordering::Greater => Self::two_sum_head(first, body, target),
        }
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted_nums = nums.into_iter().enumerate().collect::<Box<_>>();

        sorted_nums.sort_unstable_by_key(|(_, value)| *value);

        match sorted_nums
            .split_first()
            .and_then(|(first, rest)| Self::two_sum_head(*first, rest, target))
        {
            None => Vec::new(),
            Some([i, j]) => vec![i as _, j as _],
        }
    }
}

impl super::Solution for Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum(nums, target)
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
