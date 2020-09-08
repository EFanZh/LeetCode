pub struct Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let start = new_interval[0];
        let end = new_interval[1];

        let i = match intervals.binary_search_by_key(&start, |interval| interval[1]) {
            Ok(i) | Err(i) => i,
        };

        let mut j = match intervals[i..].binary_search_by_key(&end, |interval| interval[0]) {
            Ok(j) => j + 1,
            Err(j) => j,
        };

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
