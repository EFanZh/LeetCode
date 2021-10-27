pub mod two_passes;

pub trait Solution {
    fn game_of_life(board: &mut Vec<Vec<i32>>);
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]] as &dyn Matrix<_>,
            &[[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]] as &dyn Matrix<_>,
        )];

        for (board, expected) in test_cases {
            let mut board = board.to_vec();

            S::game_of_life(&mut board);

            assert_eq!(board, expected);
        }
    }
}
