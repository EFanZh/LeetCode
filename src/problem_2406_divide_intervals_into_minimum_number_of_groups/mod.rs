pub mod sweep_line;

pub trait Solution {
    fn min_groups(intervals: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[5, 10], [6, 8], [1, 5], [2, 3], [1, 10]] as &[_], 3),
            (&[[1, 3], [5, 6], [8, 10], [11, 13]], 1),
        ];

        for (intervals, expected) in test_cases {
            assert_eq!(S::min_groups(intervals.iter().map(Vec::from).collect()), expected,);
        }
    }
}
