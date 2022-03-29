pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check(line: impl Iterator<Item = char>, result: &mut i32) {
        for c in line {
            match c {
                'B' => break,
                'p' => {
                    *result += 1;

                    break;
                }
                _ => {}
            }
        }
    }

    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut result = 0;

        let (y, x) = board
            .iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().position(|&c| c == 'R').map(|x| (y, x)))
            .unwrap();

        let select_column = |row: &Vec<_>| row[x];

        Self::check(board[..y].iter().map(select_column).rev(), &mut result);
        Self::check(board[y][..x].iter().rev().copied(), &mut result);
        Self::check(board[y][x + 1..].iter().copied(), &mut result);
        Self::check(board[y + 1..].iter().map(select_column), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        Self::num_rook_captures(board)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
