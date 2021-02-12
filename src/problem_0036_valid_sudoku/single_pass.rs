pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_hits = [[false; 9]; 9];
        let mut column_hits = [[false; 9]; 9];
        let mut block_hits = [[[false; 9]; 3]; 3];

        for (i, row) in board.into_iter().enumerate() {
            let row_hits = &mut row_hits[i];

            for (j, item) in row.into_iter().enumerate() {
                if let Some(digit) = item.to_digit(10).map(|d| d as usize - 1) {
                    let row_hit = &mut row_hits[digit];

                    if *row_hit {
                        return false;
                    }

                    *row_hit = true;

                    let column_hit = &mut column_hits[j][digit];

                    if *column_hit {
                        return false;
                    }

                    *column_hit = true;

                    let block_hit = &mut block_hits[i / 3][j / 3][digit];

                    if *block_hit {
                        return false;
                    }

                    *block_hit = true;
                }
            }
        }

        true
    }
}

impl super::Solution for Solution {
    fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        Self::is_valid_sudoku(board)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
