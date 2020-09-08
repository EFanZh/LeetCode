pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|interval| interval[0]);

        let mut result = Vec::new();
        let mut iter = intervals.into_iter();

        if let Some(first) = iter.next() {
            result.push(first);

            for interval in iter {
                let previous_end = result.last_mut().unwrap().last_mut().unwrap();

                if interval[0] <= *previous_end {
                    *previous_end = (*previous_end).max(interval[1]);
                } else {
                    result.push(interval);
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
