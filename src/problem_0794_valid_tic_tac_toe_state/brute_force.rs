pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let [row_0, row_1, row_2]: &[String; 3] = board.as_slice().try_into().unwrap();
        let [cell_00, cell_01, cell_02]: [u8; 3] = row_0.as_bytes().try_into().unwrap();
        let [cell_10, cell_11, cell_12]: [u8; 3] = row_1.as_bytes().try_into().unwrap();
        let [cell_20, cell_21, cell_22]: [u8; 3] = row_2.as_bytes().try_into().unwrap();

        let mut wins = 0_u8;

        for &line in &[
            [cell_00, cell_01, cell_02],
            [cell_00, cell_10, cell_20],
            [cell_00, cell_11, cell_22],
            [cell_01, cell_11, cell_21],
            [cell_02, cell_11, cell_20],
            [cell_02, cell_12, cell_22],
            [cell_10, cell_11, cell_12],
            [cell_20, cell_21, cell_22],
        ] {
            match line {
                [b'O', b'O', b'O'] => {
                    if wins == 2 {
                        return false;
                    }

                    wins = 1;
                }
                [b'X', b'X', b'X'] => {
                    if wins == 1 {
                        return false;
                    }

                    wins = 2;
                }
                _ => {}
            }
        }

        let mut o_nums = 0;
        let mut x_nums = 0;

        for &cell in &[
            cell_00, cell_01, cell_02, cell_10, cell_11, cell_12, cell_20, cell_21, cell_22,
        ] {
            match cell {
                b'O' => o_nums += 1,
                b'X' => x_nums += 1,
                _ => {}
            }
        }

        match wins {
            0 => x_nums == o_nums || x_nums == o_nums + 1,
            1 => x_nums == o_nums,
            _ => x_nums == o_nums + 1,
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        Self::valid_tic_tac_toe(board)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
