pub mod iterative;

pub trait Solution {
    fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[0, 1, 1, 0] as &[_], &[0, 1, 1, 0], &[1, 0, 0, 1], &[1, 0, 0, 1]] as &[&[_]],
                2,
            ),
            (&[&[0, 1], &[1, 0]], 0),
            (&[&[1, 0], &[1, 0]], -1),
            (&[&[1, 1, 0], &[0, 0, 1], &[0, 0, 1]], 2),
            (&[&[1, 0, 0], &[0, 1, 1], &[1, 0, 0]], 1),
            (&[&[0, 0, 1, 1], &[1, 1, 0, 0], &[0, 1, 0, 1], &[1, 0, 1, 0]], -1),
            (&[&[1, 1, 1, 1], &[1, 1, 1, 1], &[0, 0, 0, 0], &[0, 0, 0, 0]], -1),
        ];

        for (board, expected) in test_cases {
            assert_eq!(
                S::moves_to_chessboard(board.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
