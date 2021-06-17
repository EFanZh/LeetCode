pub mod bfs;
pub mod bfs_2;

pub trait Solution {
    fn solve(board: &mut Vec<Vec<char>>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &["XXXX", "XOOX", "XXOX", "XOXX"] as &[&str],
                &["XXXX", "XXXX", "XXXX", "XOXX"] as &[_],
            ),
            (
                &["OXXXO", "XOXOO", "OOXOX", "XXXXX"],
                &["OXXXO", "XOXOO", "OOXOX", "XXXXX"],
            ),
            (&["X"], &["X"]),
        ];

        for (board, expected) in test_cases {
            let mut board = board.iter().map(|row| row.chars().collect()).collect();

            S::solve(&mut board);

            assert_eq!(
                board
                    .into_iter()
                    .map(|row| row.into_iter().collect::<String>())
                    .collect::<Box<_>>()
                    .as_ref(),
                expected
            );
        }
    }
}
