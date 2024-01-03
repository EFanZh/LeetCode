pub mod greedy;

pub trait Solution {
    fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 4], [4, 8]] as &[_], 8),
            (&[[1, 3], [2, 4], [10, 11], [10, 12], [8, 9]], 32),
            (&[[1, 7], [2, 8], [3, 9], [4, 10], [5, 11], [6, 12]], 27),
        ];

        for (tasks, expected) in test_cases {
            assert_eq!(S::minimum_effort(tasks.iter().map(Vec::from).collect()), expected);
        }
    }
}
