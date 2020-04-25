pub mod greedy;

pub trait Solution {
    fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[(1, 2), (2, 3), (3, 4), (1, 3)] as &[_], 1),
            (&[(1, 2), (1, 2), (1, 2)], 2),
            (&[(1, 2), (2, 3)], 0),
        ];

        for (intervals, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::erase_overlap_intervals(intervals.iter().map(|&(s, f)| vec![s, f]).collect()),
                expected
            );
        }
    }
}
