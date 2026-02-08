pub mod dynamic_programming;

pub trait Solution {
    fn max_score(n: i32, k: i32, stay_score: Vec<Vec<i32>>, travel_score: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (2, 1, &[[2, 3]] as &dyn Matrix<_>, &[[0, 2], [1, 0]] as &dyn Matrix<_>),
                3,
            ),
            ((3, 2, &[[3, 4, 2], [2, 1, 2]], &[[0, 2, 1], [2, 0, 4], [3, 2, 0]]), 8),
        ];

        for ((n, k, stay_score, travel_score), expected) in test_cases {
            assert_eq!(S::max_score(n, k, stay_score.to_vec(), travel_score.to_vec()), expected);
        }
    }
}
