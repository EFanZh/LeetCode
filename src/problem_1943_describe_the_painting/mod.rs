pub mod iterative;

pub trait Solution {
    fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 4, 5], [4, 7, 7], [1, 7, 9]] as &[_],
                &[[1_i64, 4, 14], [4, 7, 16]] as &[_],
            ),
            (
                &[[1, 7, 9], [6, 8, 15], [8, 10, 7]],
                &[[1, 6, 9], [6, 7, 24], [7, 8, 15], [8, 10, 7]],
            ),
            (
                &[[1, 4, 5], [1, 4, 7], [4, 7, 1], [4, 7, 11]],
                &[[1, 4, 12], [4, 7, 12]],
            ),
        ];

        for (segments, expected) in test_cases {
            assert_eq!(S::split_painting(segments.iter().map(Vec::from).collect()), expected);
        }
    }
}
