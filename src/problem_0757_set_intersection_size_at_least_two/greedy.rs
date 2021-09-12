pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::convert::TryInto;

impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals
            .into_iter()
            .map(|interval| {
                let [left, right]: [_; 2] = interval.as_slice().try_into().unwrap();

                (left, right)
            })
            .collect::<Vec<_>>();

        intervals.sort_unstable_by_key(|&(left, right)| (right, Reverse(left)));

        let mut result = 0;
        let mut prev_1 = i32::MIN;
        let mut prev_2 = i32::MIN;

        for (left, right) in intervals {
            if left > prev_2 {
                result += 2;
                prev_1 = right - 1;
                prev_2 = right;
            } else if left > prev_1 {
                result += 1;
                prev_1 = prev_2;
                prev_2 = right;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        Self::intersection_size_two(intervals)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
