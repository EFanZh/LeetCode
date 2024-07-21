pub mod brute_force;

pub trait Solution {
    fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    4,
                    &[[1, 2, 3], [3, 2, 0], [3, 1, 0], [1, 2, 0]] as &dyn Matrix<_>,
                    &[[0, 1], [2, 3]] as &[_],
                ),
                2,
            ),
            ((2, &[[1], [0]], &[[1, 0]]), 0),
            ((4, &[[1, 3, 2], [2, 3, 0], [1, 3, 0], [0, 2, 1]], &[[1, 3], [0, 2]]), 4),
        ];

        for ((n, preferences, pairs), expected) in test_cases {
            assert_eq!(
                S::unhappy_friends(n, preferences.to_vec(), pairs.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
