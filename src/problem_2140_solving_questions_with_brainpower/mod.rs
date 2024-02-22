pub mod dynamic_programming;

pub trait Solution {
    fn most_points(questions: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[3, 2], [4, 3], [4, 4], [2, 5]] as &[_], 5),
            (&[[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]], 7),
        ];

        for (questions, expected) in test_cases {
            assert_eq!(S::most_points(questions.iter().map(Vec::from).collect()), expected);
        }
    }
}
