pub mod backtracking_1;
pub mod backtracking_2;

pub trait Solution {
    fn solve_sudoku(board: &mut Vec<Vec<char>>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
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
            [
                "534678912",
                "672195348",
                "198342567",
                "859761423",
                "426853791",
                "713924856",
                "961537284",
                "287419635",
                "345286179",
            ],
        )];

        for (board, expected) in test_cases {
            let mut board = board.iter().map(|row| row.chars().collect()).collect();

            let expected = expected
                .iter()
                .map(|row| row.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            S::solve_sudoku(&mut board);

            assert_eq!(board, expected);
        }
    }
}
