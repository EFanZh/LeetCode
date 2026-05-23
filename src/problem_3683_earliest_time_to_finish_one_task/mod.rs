pub mod iterative;

pub trait Solution {
    fn earliest_time(tasks: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 6], [2, 3]] as &[_], 5),
            (&[[100, 100], [100, 100], [100, 100]], 200),
        ];

        for (tasks, expected) in test_cases {
            assert_eq!(S::earliest_time(tasks.iter().map(Vec::from).collect()), expected);
        }
    }
}
