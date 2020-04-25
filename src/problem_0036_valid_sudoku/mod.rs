pub mod naive;
pub mod single_pass;

pub trait Solution {
    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                [
                    "53..7....",
                    "6..195...",
                    ".98....6.",
                    "8...6...3",
                    "4..8.3..1",
                    "7...2...6",
                    ".6....28.",
                    "...419..5",
                    "....8..79",
                ],
                true,
            ),
            (
                [
                    "83..7....",
                    "6..195...",
                    ".98....6.",
                    "8...6...3",
                    "4..8.3..1",
                    "7...2...6",
                    ".6....28.",
                    "...419..5",
                    "....8..79",
                ],
                false,
            ),
        ];

        for (board, expected) in test_cases.iter().copied() {
            let result = S::is_valid_sudoku(board.iter().map(|row| row.chars().collect()).collect());

            assert_eq!(result, expected);
        }
    }
}
