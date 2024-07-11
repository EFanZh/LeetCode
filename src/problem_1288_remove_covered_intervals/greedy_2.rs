pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut sorted_intervals = intervals
            .into_iter()
            .map(|interval| {
                let [left, right]: [_; 2] = interval.try_into().ok().unwrap();

                (left as u32, right as u32)
            })
            .collect::<Vec<_>>();

        sorted_intervals.sort_unstable_by_key(|&(left, right)| (left, Reverse(right)));

        let mut max = 0;
        let mut result = 0;

        for (_, right) in sorted_intervals {
            if right > max {
                max = right;
                result += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        Self::remove_covered_intervals(intervals)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
