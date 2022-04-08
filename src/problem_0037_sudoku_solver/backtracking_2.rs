pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn solve_sudoku_helper(
        board: &mut [Vec<char>],
        row_hits: &mut [bool; 81],
        column_hits: &mut [bool; 81],
        block_hits: &mut [bool; 81],
        i: usize,
    ) -> Result<(), ()> {
        if i == 81 {
            Err(())
        } else {
            let row = i / 9;
            let column = i % 9;
            let block = row / 3 * 3 + column / 3;
            let row_hit_start = row * 9;
            let column_hit_start = column * 9;
            let block_hit_start = block * 9;

            if board[row][column] == '.' {
                for slot in 0_u8..9_u8 {
                    let usize_slot = slot as usize;
                    let row_hit_index = row_hit_start + usize_slot;
                    let column_hit_index = column_hit_start + usize_slot;
                    let block_hit_index = block_hit_start + usize_slot;

                    if !(row_hits[row_hit_index] || column_hits[column_hit_index] || block_hits[block_hit_index]) {
                        board[row][column] = char::from(b'1' + slot);
                        row_hits[row_hit_index] = true;
                        column_hits[column_hit_index] = true;
                        block_hits[block_hit_index] = true;

                        Self::solve_sudoku_helper(board, row_hits, column_hits, block_hits, i + 1)?;

                        row_hits[row_hit_index] = false;
                        column_hits[column_hit_index] = false;
                        block_hits[block_hit_index] = false;
                    }
                }

                board[row][column] = '.';

                Ok(())
            } else {
                Self::solve_sudoku_helper(board, row_hits, column_hits, block_hits, i + 1)
            }
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row_hits = [false; 81];
        let mut column_hits = [false; 81];
        let mut block_hits = [false; 81];

        for (i, row) in board.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if let Some(num) = item.to_digit(10) {
                    let k = (num - 1) as usize;

                    row_hits[9 * i + k] = true;
                    column_hits[9 * j + k] = true;
                    block_hits[9 * (i / 3 * 3 + j / 3) + k] = true;
                }
            }
        }

        Self::solve_sudoku_helper(board, &mut row_hits, &mut column_hits, &mut block_hits, 0).unwrap_err();
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
