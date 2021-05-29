pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut result = 0;

        for (i, row) in board.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                result += i32::from(
                    cell == 'X'
                        && board.get(i.wrapping_sub(1)).map_or(true, |values| values[j] == '.')
                        && row.get(j.wrapping_sub(1)).map_or(true, |&c| c == '.')
                        && (row.get(j + 1).map_or(true, |&c| c == '.')
                            || board.get(i + 1).map_or(true, |values| values[j] == '.')),
                );
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        Self::count_battleships(board)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
