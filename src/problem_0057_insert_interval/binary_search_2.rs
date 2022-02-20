pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        let start = new_interval[0];
        let end = new_interval[1];
        let i = intervals.partition_point(|interval| interval[1] < start);
        let mut j = intervals[i..].partition_point(|interval| interval[0] <= end);

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
