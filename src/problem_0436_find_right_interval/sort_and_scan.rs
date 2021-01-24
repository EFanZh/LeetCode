pub struct Solution;

use std::cmp::Reverse;
use std::convert::TryInto;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut endpoints = Vec::with_capacity(n * 2);

        for (i, interval) in (0..).zip(intervals) {
            let [start, end]: [i32; 2] = interval.as_slice().try_into().unwrap();

            endpoints.extend(&[(end, false, i), (start, true, i)]);
        }

        endpoints.sort_unstable_by_key(|&(position, is_start, _)| Reverse((position, is_start)));

        let mut result = vec![0; n];
        let mut right = -1;

        for (_, is_start, index) in endpoints {
            if is_start {
                right = index;
            } else {
                result[index as usize] = right;
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        Self::find_right_interval(intervals)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
