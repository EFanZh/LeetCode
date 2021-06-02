pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::convert::TryInto;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();

        let (mut lefts, mut rights): (Vec<_>, Vec<_>) = (0..)
            .zip(intervals)
            .map(|(i, interval)| {
                let [start, end]: [i32; 2] = interval.as_slice().try_into().unwrap();

                ((start, i), (end, i))
            })
            .unzip();

        lefts.sort_unstable_by_key(|&(position, _)| Reverse(position));
        rights.sort_unstable_by_key(|&(position, _)| Reverse(position));

        let mut result = vec![0; n];
        let mut prev = -1;
        let mut left_iter = lefts.into_iter();
        let mut right_iter = rights.into_iter();

        let (mut left, mut i) = left_iter.next().unwrap();
        let (mut right, mut j) = right_iter.next().unwrap();

        loop {
            if left < right {
                result[j as usize] = prev;

                if let Some((next_right, next_j)) = right_iter.next() {
                    right = next_right;
                    j = next_j;
                } else {
                    break;
                }
            } else {
                prev = i;

                if let Some((next_left, next_i)) = left_iter.next() {
                    left = next_left;
                    i = next_i;
                } else {
                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
