pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    fn is_legal(board: &[u8; 64], mut location: (u8, u8), step: (u8, u8), middle: u8, end: u8) -> bool {
        let mut iter = iter::from_fn(|| {
            location = (location.0.wrapping_add(step.0), location.1.wrapping_add(step.1));

            (location.0 < 8 && location.1 < 8).then(|| board[usize::from(location.0 * 8 + location.1)])
        });

        if iter.next() == Some(middle) {
            for value in iter {
                if value == end {
                    return true;
                } else if value != middle {
                    break;
                }
            }
        }

        false
    }

    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let mut board_2 = [0_u8; 64];

        for (target, cell) in board_2.iter_mut().zip(board.into_iter().flatten()) {
            *target = cell as _;
        }

        let location = (r_move as _, c_move as _);
        let (middle, end) = if color == 'B' { (b'W', b'B') } else { (b'B', b'W') };

        for step in [
            (u8::MAX, u8::MAX),
            (u8::MAX, 0),
            (u8::MAX, 1),
            (0, u8::MAX),
            (0, 1),
            (1, u8::MAX),
            (1, 0),
            (1, 1),
        ] {
            if Self::is_legal(&board_2, location, step, middle, end) {
                return true;
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        Self::check_move(board, r_move, c_move, color)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
