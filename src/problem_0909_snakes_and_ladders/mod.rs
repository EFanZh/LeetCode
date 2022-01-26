pub mod bfs;

pub trait Solution {
    fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    [-1, -1, -1, -1, -1, -1],
                    [-1, -1, -1, -1, -1, -1],
                    [-1, -1, -1, -1, -1, -1],
                    [-1, 35, -1, -1, 13, -1],
                    [-1, -1, -1, -1, -1, -1],
                    [-1, 15, -1, -1, -1, -1],
                ] as &dyn Matrix<_>,
                4,
            ),
            (&[[-1, -1], [-1, 3]], 1),
            (&[[1, 1, -1], [1, 1, 1], [-1, 1, 1]], -1),
        ];

        for (board, expected) in test_cases {
            assert_eq!(S::snakes_and_ladders(board.to_vec()), expected);
        }
    }
}
