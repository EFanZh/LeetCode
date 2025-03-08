pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;
use std::slice::Iter;

enum Piece {
    Queen,
    Bishop,
    Rook,
}

impl Solution {
    fn helper(board: &[Cell<u8>; 64], mut pieces: Iter<(Piece, u8, u8)>, result: &mut u32) {
        const MOVES: [(u8, u8); 8] = [
            (u8::MAX, u8::MAX),
            (u8::MAX, 1),
            (1, u8::MAX),
            (1, 1),
            (u8::MAX, 0),
            (0, u8::MAX),
            (0, 1),
            (1, 0),
        ];

        if let Some((piece, row, column)) = pieces.next() {
            let (row, column) = (*row, *column);

            // Not moving.

            let cell = &board[usize::from(8 * row + column)];

            if cell.get() == 0 {
                cell.set(u8::MAX);
                Self::helper(board, pieces.clone(), result);
                cell.set(0);
            }

            let directions = match piece {
                Piece::Queen => &MOVES,
                Piece::Bishop => &MOVES[..4],
                Piece::Rook => &MOVES[4..],
            };

            for &(row_direction, column_direction) in directions {
                let mut row = row;
                let mut column = column;
                let mut time = 1;

                loop {
                    row = row.wrapping_add(row_direction);
                    column = column.wrapping_add(column_direction);

                    if row < 8 && column < 8 {
                        let cell = &board[usize::from(8 * row + column)];
                        let saved_cell = cell.get();

                        if saved_cell & time == 0 {
                            let time_till_end = !(time - 1);

                            if saved_cell & time_till_end == 0 {
                                cell.set(saved_cell | time_till_end);
                                Self::helper(board, pieces.clone(), result);
                            }

                            cell.set(saved_cell | time);
                            time <<= 1;

                            continue;
                        }
                    }

                    break;
                }

                loop {
                    time >>= 1;

                    if time == 0 {
                        break;
                    }

                    row = row.wrapping_sub(row_direction);
                    column = column.wrapping_sub(column_direction);

                    let cell = &board[usize::from(8 * row + column)];

                    cell.set(cell.get() ^ time);
                }
            }
        } else {
            *result += 1;
        }
    }

    pub fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
        let pieces = pieces
            .into_iter()
            .zip(positions)
            .map(|(piece, position)| {
                let piece = match piece.into_bytes().first() {
                    Some(b'b') => Piece::Bishop,
                    Some(b'q') => Piece::Queen,
                    _ => Piece::Rook,
                };

                let [row, column] = position.try_into().ok().unwrap();

                (piece, row as u8 - 1, column as u8 - 1)
            })
            .collect::<Box<_>>();

        let mut result = 0;

        Self::helper(&[const { Cell::new(0) }; 64], pieces.iter(), &mut result);

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_combinations(pieces: Vec<String>, positions: Vec<Vec<i32>>) -> i32 {
        Self::count_combinations(pieces, positions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
