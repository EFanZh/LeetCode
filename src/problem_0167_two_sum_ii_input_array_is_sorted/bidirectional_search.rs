pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn two_sum_head(index: usize, first: i32, rest: &[i32], target: i32) -> Option<[usize; 2]> {
        rest.split_last()
            .and_then(|(last, body)| Self::dispatch(index, first, *last, body, target))
    }

    fn two_sum_tail(index: usize, last: i32, rest: &[i32], target: i32) -> Option<[usize; 2]> {
        rest.split_first()
            .and_then(|(first, body)| Self::dispatch(index, *first, last, body, target))
    }

    fn dispatch(index: usize, first: i32, last: i32, body: &[i32], target: i32) -> Option<[usize; 2]> {
        match (first + last).cmp(&target) {
            Ordering::Less => Self::two_sum_tail(index + 1, last, body, target),
            Ordering::Equal => Some([index, index + body.len() + 1]),
            Ordering::Greater => Self::two_sum_head(index, first, body, target),
        }
    }

    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        match numbers
            .split_first()
            .and_then(|(first, rest)| Self::two_sum_head(0, *first, rest, target))
        {
            None => Vec::new(),
            Some([i, j]) => vec![(i + 1) as _, (j + 1) as _],
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        Self::two_sum(numbers, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
