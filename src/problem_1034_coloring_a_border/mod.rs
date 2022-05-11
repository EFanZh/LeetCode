pub mod bfs;

pub trait Solution {
    fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[1, 1], [1, 2]] as &dyn Matrix<_>, 0, 0, 3),
                &[[3, 3], [3, 2]] as &dyn Matrix<_>,
            ),
            ((&[[1, 2, 2], [2, 3, 2]], 0, 1, 3), &[[1, 3, 3], [2, 3, 3]]),
            (
                (&[[1, 1, 1], [1, 1, 1], [1, 1, 1]], 1, 1, 2),
                &[[2, 2, 2], [2, 1, 2], [2, 2, 2]],
            ),
        ];

        for ((grid, row, col, color), expected) in test_cases {
            assert_eq!(S::color_border(grid.to_vec(), row, col, color), expected);
        }
    }
}
