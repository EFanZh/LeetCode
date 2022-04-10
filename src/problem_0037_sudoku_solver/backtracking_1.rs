pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn solve_sudoku_helper(board: &mut [Vec<char>], i: usize) -> Result<(), ()> {
        if i == 81 {
            Err(())
        } else {
            let row = i / 9;
            let column = i % 9;

            if board[row][column] == '.' {
                let block_row = row / 3 * 3;
                let block_column = column / 3 * 3;

                for num in (b'1'..=b'9').map(char::from) {
                    if !board[row].contains(&num)
                        && (0..9).all(|i| board[i][column] != num)
                        && board[block_row..block_row + 3]
                            .iter()
                            .all(|r| !r[block_column..block_column + 3].contains(&num))
                    {
                        board[row][column] = num;

                        Self::solve_sudoku_helper(board, i + 1)?;
                    }
                }

                board[row][column] = '.';

                Ok(())
            } else {
                Self::solve_sudoku_helper(board, i + 1)
            }
        }
    }

    #[allow(clippy::ptr_arg)] // Expected.
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve_sudoku_helper(board, 0).unwrap_err();
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve_sudoku(board);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
