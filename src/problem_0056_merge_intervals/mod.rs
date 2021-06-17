pub mod greedy;
pub mod greedy_2;

pub trait Solution {
    fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 3], [2, 6], [8, 10], [15, 18]] as &[_],
                &[[1, 6], [8, 10], [15, 18]] as &[_],
            ),
            (&[[1, 4], [4, 5]], &[[1, 5]]),
            (&[[1, 4], [0, 4]], &[[0, 4]]),
            (&[[1, 4], [2, 3]], &[[1, 4]]),
        ];

        for (intervals, expected) in test_cases {
            assert_eq!(S::merge(intervals.iter().copied().map(Vec::from).collect()), expected);
        }
    }
}
