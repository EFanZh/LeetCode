pub mod sweep_line;

pub trait Solution {
    fn count_intersecting_intervals(intervals: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 3], [3, 4]] as &[_], 2),
            (&[[1, 5], [2, 4], [3, 6]], 3),
            (&[[1, 2], [3, 4], [5, 6]], 0),
        ];

        for (intervals, expected) in test_cases {
            assert_eq!(
                S::count_intersecting_intervals(intervals.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
