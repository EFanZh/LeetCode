pub struct Solution {}

impl Solution {
    fn exist_helper(board: &mut [Vec<char>], word: &[u8], columns: usize, row: usize, column: usize) -> bool {
        if let Some((c, rest_word)) = word.split_first() {
            let c = char::from(*c);

            if row < board.len() && column < columns && board[row][column] == c {
                board[row][column] = '✓';

                if Self::exist_helper(board, rest_word, columns, row, column.wrapping_sub(1))
                    || Self::exist_helper(board, rest_word, columns, row.wrapping_sub(1), column)
                    || Self::exist_helper(board, rest_word, columns, row, column + 1)
                    || Self::exist_helper(board, rest_word, columns, row + 1, column)
                {
                    return true;
                }

                board[row][column] = c;
            }

            false
        } else {
            true
        }
    }

    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let columns = board.first().map_or(0, Vec::len);

        for row in 0..board.len() {
            for column in 0..columns {
                if Self::exist_helper(&mut board, word.as_bytes(), columns, row, column) {
                    return true;
                }
            }
        }

        false
    }
}

impl super::Solution for Solution {
    fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        Self::exist(board, word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
