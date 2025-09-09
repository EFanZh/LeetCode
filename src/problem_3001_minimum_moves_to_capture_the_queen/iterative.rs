#![expect(clippy::many_single_char_names, reason = "by design")]

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn check_one_step<K1, K2>(
        this: (u8, u8),
        that: (u8, u8),
        queen: (u8, u8),
        mut key: impl FnMut(u8, u8) -> K1,
        mut perpendicular_key: impl FnMut(u8, u8) -> K2,
    ) -> bool
    where
        K1: Ord,
        K2: Ord,
    {
        let this_key = key(this.0, this.1);
        let that_key = key(that.0, that.1);
        let queen_key = key(queen.0, queen.1);

        this_key == queen_key
            && (this_key != that_key || {
                let mut this_perpendicular_key = perpendicular_key(this.0, this.1);
                let that_perpendicular_key = perpendicular_key(that.0, that.1);
                let mut queen_perpendicular_key = perpendicular_key(queen.0, queen.1);

                if queen_perpendicular_key < this_perpendicular_key {
                    mem::swap(&mut this_perpendicular_key, &mut queen_perpendicular_key);
                }

                that_perpendicular_key < this_perpendicular_key || that_perpendicular_key > queen_perpendicular_key
            })
    }

    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        let (rook, bishop, queen) = ((a as u8, b as u8), (c as u8, d as u8), (e as u8, f as u8));
        let row_key = |row, _| row;
        let column_key = |_, column| column;
        let diagonal_key = |row: u8, column: u8| row.cast_signed() - column.cast_signed();
        let anti_diagonal_key = |row, column| row + column;

        if Self::check_one_step(rook, bishop, queen, row_key, column_key)
            || Self::check_one_step(rook, bishop, queen, column_key, row_key)
            || Self::check_one_step(bishop, rook, queen, diagonal_key, anti_diagonal_key)
            || Self::check_one_step(bishop, rook, queen, anti_diagonal_key, diagonal_key)
        {
            1
        } else {
            2
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        Self::min_moves_to_capture_the_queen(a, b, c, d, e, f)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
