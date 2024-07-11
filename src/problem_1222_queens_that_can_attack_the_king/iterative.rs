pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut board = 0_u64;

        for queen in queens.iter().map(Vec::as_slice) {
            let [y, x]: [_; 2] = queen.try_into().unwrap();

            board |= 1 << (8 * y + x);
        }

        let [king_y, king_x]: [_; 2] = king.try_into().unwrap();
        let (king_y, king_x) = (king_y as u32, king_x as u32);
        let mut result = queens;
        let mut result_length = 0;

        for (y_diff, x_diff) in [
            (u32::MAX, u32::MAX),
            (u32::MAX, 0),
            (u32::MAX, 1),
            (0, u32::MAX),
            (0, 1),
            (1, u32::MAX),
            (1, 0),
            (1, 1),
        ] {
            let mut y = king_y;
            let mut x = king_x;

            loop {
                y = y.wrapping_add(y_diff);
                x = x.wrapping_add(x_diff);

                if y < 8 && x < 8 {
                    if board & (1 << (8 * y + x)) != 0 {
                        result[result_length].splice(.., [y as i32, x as i32]);
                        result_length += 1;

                        break;
                    }
                } else {
                    break;
                }
            }
        }

        result.truncate(result_length);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        Self::queens_attackthe_king(queens, king)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
