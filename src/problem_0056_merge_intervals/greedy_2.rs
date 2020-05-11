pub struct Solution {}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals
            .into_iter()
            .map(|interval| (interval[0], interval[1]))
            .collect::<Vec<_>>();

        intervals.sort_unstable_by_key(|interval| interval.0);

        let mut result = Vec::new();
        let mut iter = intervals.into_iter();

        if let Some(first) = iter.next() {
            result.push(vec![first.0, first.1]);

            for interval in iter {
                let previous_end = result.last_mut().unwrap().last_mut().unwrap();

                if interval.0 <= *previous_end {
                    *previous_end = (*previous_end).max(interval.1);
                } else {
                    result.push(vec![interval.0, interval.1]);
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::merge(intervals)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
