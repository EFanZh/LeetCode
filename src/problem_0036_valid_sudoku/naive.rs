pub struct Solution;

impl Solution {
    fn has_duplication<I: IntoIterator<Item = char>>(iter: I) -> bool {
        let mut count = [false; 9];

        for slot in iter {
            if let Some(digit) = slot.to_digit(10) {
                let target = &mut count[digit as usize - 1];

                if *target {
                    return true;
                } else {
                    *target = true;
                }
            }
        }

        false
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for row in &board {
            if Self::has_duplication(row.iter().copied()) {
                return false;
            }
        }

        for column in (0..9).map(|i| board.iter().map(move |row| row[i])) {
            if Self::has_duplication(column) {
                return false;
            }
        }

        for block_row in (0..9).step_by(3) {
            for block_column in (0..9).step_by(3) {
                if Self::has_duplication(
                    board[block_row..block_row + 3]
                        .iter()
                        .flat_map(|row| row[block_column..block_column + 3].iter().copied()),
                ) {
                    return false;
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
