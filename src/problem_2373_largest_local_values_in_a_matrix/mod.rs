pub mod iterative;

pub trait Solution {
    fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[9, 9, 8, 1], [5, 6, 2, 6], [8, 2, 6, 4], [6, 2, 2, 2]] as &dyn Matrix<_>,
                &[[9, 9], [8, 6]] as &dyn Matrix<_>,
            ),
            (
                &[
                    [1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1],
                    [1, 1, 2, 1, 1],
                    [1, 1, 1, 1, 1],
                    [1, 1, 1, 1, 1],
                ],
                &[[2, 2, 2], [2, 2, 2], [2, 2, 2]],
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::largest_local(grid.to_vec()), expected);
        }
    }
}
