pub mod iterative;

pub trait Solution {
    fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 2, 3], [3, 1, 5], [3, 2, 1]] as &dyn Matrix<_>,
                &[[1, 1, 0], [1, 0, 1], [0, 1, 1]] as &dyn Matrix<_>,
            ),
            (&[[1]], &[[0]]),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::difference_of_distinct_values(grid.to_vec()), expected);
        }
    }
}
