pub mod iterative;

pub trait Solution {
    fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[0, 1, 1, 0], [0, 1, 1, 0], [1, 0, 0, 1], [1, 0, 0, 1]] as &dyn Matrix<_>,
                2,
            ),
            (&[[0, 1], [1, 0]], 0),
            (&[[1, 0], [1, 0]], -1),
            (&[[1, 1, 0], [0, 0, 1], [0, 0, 1]], 2),
            (&[[1, 0, 0], [0, 1, 1], [1, 0, 0]], 1),
            (&[[0, 0, 1, 1], [1, 1, 0, 0], [0, 1, 0, 1], [1, 0, 1, 0]], -1),
            (&[[1, 1, 1, 1], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]], -1),
        ];

        for (board, expected) in test_cases {
            assert_eq!(S::moves_to_chessboard(board.to_vec()), expected);
        }
    }
}
