pub mod two_passes;

pub trait Solution {
    fn game_of_life(board: &mut Vec<Vec<i32>>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[&[0, 1, 0] as &[_], &[0, 0, 1], &[1, 1, 1], &[0, 0, 0]] as &[&[_]],
            &[&[0, 0, 0], &[1, 0, 1], &[0, 1, 1], &[0, 1, 0]],
        )];

        for (board, expected) in test_cases.iter().copied() {
            let mut board = board.iter().map(|row| row.to_vec()).collect();

            S::game_of_life(&mut board);

            assert_eq!(board, expected);
        }
    }
}
