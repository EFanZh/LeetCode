pub mod sort_and_merge;
pub mod sort_and_scan;

pub trait Solution {
    fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2]] as &[_], &[-1] as &[_]),
            (&[[3, 4], [2, 3], [1, 2]], &[-1, 0, 1]),
            (&[[1, 4], [2, 3], [3, 4]], &[-1, 2, -1]),
        ];

        for (intervals, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::find_right_interval(intervals.iter().map(|interval| interval.to_vec()).collect()),
                expected
            );
        }
    }
}
