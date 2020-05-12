pub struct Solution {}

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let start = new_interval[0];
        let end = new_interval[1];

        match intervals.binary_search_by(|interval| interval[1].cmp(&start)) {
            Ok(i) => match intervals[i..].binary_search_by(|interval| interval[0].cmp(&end)) {
                Ok(mut j) => {
                    j += i;
                    intervals[i][1] = intervals[j][1];
                    intervals.drain(i + 1..=j);
                }
                Err(mut j) => {
                    j += i;
                    intervals[i][1] = end.max(intervals[j - 1][1]);
                    intervals.drain(i + 1..j);
                }
            },
            Err(i) => match intervals[i..].binary_search_by(|interval| interval[0].cmp(&end)) {
                Ok(mut j) => {
                    j += i;
                    intervals[j][0] = start.min(intervals[i][0]);
                    intervals.drain(i..j);
                }
                Err(mut j) => {
                    if j == 0 {
                        intervals.insert(i, new_interval);
                    } else {
                        j += i;
                        intervals[i][0] = start.min(intervals[i][0]);
                        intervals[i][1] = end.max(intervals[j - 1][1]);
                        intervals.drain(i + 1..j);
                    }
                }
            },
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
