pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::convert::TryInto;

impl Solution {
    fn neighbors(board: &mut [Vec<char>], (row, column): (usize, usize), mut f: impl FnMut((usize, usize), &mut char)) {
        for &(r, c) in &[
            (row.wrapping_sub(1), column.wrapping_sub(1)),
            (row.wrapping_sub(1), column),
            (row.wrapping_sub(1), column + 1),
            (row, column.wrapping_sub(1)),
            (row, column + 1),
            (row + 1, column.wrapping_sub(1)),
            (row + 1, column),
            (row + 1, column + 1),
        ] {
            if let Some(slot) = board.get_mut(r).and_then(|row| row.get_mut(c)) {
                f((r, c), slot);
            }
        }
    }

    fn count_mines(board: &mut [Vec<char>], position: (usize, usize)) -> u8 {
        let mut result = 0;

        Self::neighbors(board, position, |_, slot| {
            if *slot == 'M' {
                result += 1;
            }
        });

        result
    }

    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut board = board;
        let [row, column]: [i32; 2] = click.as_slice().try_into().unwrap();
        let mut position = (row as usize, column as usize);
        let slot = &mut board[position.0][position.1];

        if *slot == 'E' {
            *slot = 'V';

            let mut queue = VecDeque::new();

            loop {
                let count = Self::count_mines(&mut board, position);

                board[position.0][position.1] = if count == 0 {
                    Self::neighbors(&mut board, position, |next, slot| {
                        if *slot == 'E' {
                            *slot = 'V';
                            queue.push_back(next);
                        }
                    });

                    'B'
                } else {
                    char::from(b'0' + count)
                };

                if let Some(next) = queue.pop_front() {
                    position = next;
                } else {
                    break;
                }
            }
        } else {
            *slot = 'X';
        }

        board
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        Self::update_board(board, click)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
