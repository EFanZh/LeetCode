pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn two_sum_head(first: (i32, i32), rest: &[(i32, i32)], target: i32) -> Option<[i32; 2]> {
        rest.split_last()
            .and_then(|(last, body)| Self::dispatch(first, *last, body, target))
    }

    fn two_sum_tail(last: (i32, i32), rest: &[(i32, i32)], target: i32) -> Option<[i32; 2]> {
        rest.split_first()
            .and_then(|(first, body)| Self::dispatch(*first, last, body, target))
    }

    fn dispatch(first: (i32, i32), last: (i32, i32), body: &[(i32, i32)], target: i32) -> Option<[i32; 2]> {
        match (first.1 + last.1).cmp(&target) {
            Ordering::Less => Self::two_sum_tail(last, body, target),
            Ordering::Equal => Some([first.0, last.0]),
            Ordering::Greater => Self::two_sum_head(first, body, target),
        }
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = (0..).zip(nums).collect::<Box<_>>();

        nums.sort_unstable_by_key(|(_, value)| *value);

        nums.split_first()
            .and_then(|(first, rest)| Self::two_sum_head(*first, rest, target))
            .map(|[i, j]| vec![i, j])
            .unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
