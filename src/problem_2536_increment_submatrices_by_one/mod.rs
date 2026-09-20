pub mod prefix_sums;

pub trait Solution {
    fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (3, &[[1, 1, 2, 2], [0, 0, 1, 1]] as &[_]),
                &[[1, 1, 0], [1, 2, 1], [0, 1, 1]] as &dyn Matrix<_>,
            ),
            ((2, &[[0, 0, 1, 1]]), &[[1, 1], [1, 1]]),
        ];

        for ((n, queries), expected) in test_cases {
            assert_eq!(
                S::range_add_queries(n, queries.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
