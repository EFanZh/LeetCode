pub mod iterative;

pub trait Solution {
    fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1], [22], [333]] as &dyn Matrix<_>, &[3] as &[_]),
            (&[[-15, 1, 3], [15, 7, 12], [5, 6, -2]], &[3, 1, 2]),
            (
                &[
                    [-4, 0, -1, 3, 9, 8, 6],
                    [-2, 5, -5, -7, -2, -6, 7],
                    [4, -4, 1, 7, 0, 6, 8],
                    [-6, 2, -5, 0, 9, 1, -6],
                ],
                &[2, 2, 2, 2, 2, 2, 2],
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::find_column_width(grid.to_vec()), expected);
        }
    }
}
