pub mod iterative;

pub trait Solution {
    fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&["EEEEE", "EEMEE", "EEEEE", "EEEEE"] as &[_], [3, 0]),
                &["B1E1B", "B1M1B", "B111B", "BBBBB"] as &[_],
            ),
            (
                (&["B1E1B", "B1M1B", "B111B", "BBBBB"], [1, 2]),
                &["B1E1B", "B1X1B", "B111B", "BBBBB"],
            ),
        ];

        for ((board, click), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::update_board(board.iter().map(|row| row.chars().collect()).collect(), click.to_vec())
                    .into_iter()
                    .map(|row| row.into_iter().collect::<String>())
                    .collect::<Box<_>>()
                    .as_ref(),
                expected
            );
        }
    }
}
