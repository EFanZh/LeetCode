pub mod greedy;

pub trait Solution {
    fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 3], [1, 4], [2, 5], [3, 5]] as &[_], 3),
            (&[[1, 2], [2, 3], [2, 4], [4, 5]], 5),
            (&[[1, 3], [3, 7], [5, 7], [7, 8]], 5),
        ];

        for (intervals, expected) in test_cases {
            assert_eq!(
                S::intersection_size_two(intervals.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
