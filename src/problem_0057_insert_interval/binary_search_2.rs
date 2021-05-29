pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn lower_bound_by<T: Ord, F: FnMut(&T) -> bool>(values: &[T], mut f: F) -> usize {
        values
            .binary_search_by(move |value| if f(value) { Ordering::Less } else { Ordering::Greater })
            .unwrap_err()
    }

    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let start = new_interval[0];
        let end = new_interval[1];

        let i = Self::lower_bound_by(&intervals, |interval| interval[1] < start);
        let mut j = Self::lower_bound_by(&intervals[i..], |interval| interval[0] <= end);

        if j == 0 {
            intervals.insert(i, new_interval);
        } else {
            j += i;
            intervals[i][0] = intervals[i][0].min(start);
            intervals[i][1] = intervals[j - 1][1].max(end);

            intervals.drain(i + 1..j);
        }

        intervals
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        Self::insert(intervals, new_interval)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
