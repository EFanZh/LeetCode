pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 4], [3, 6], [2, 8]] as &[_], 2),
            (&[[1, 4], [2, 3]], 1),
            (&[[3, 10], [4, 10], [5, 11]], 2),
        ];

        for (intervals, expected) in test_cases {
            assert_eq!(
                S::remove_covered_intervals(intervals.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
