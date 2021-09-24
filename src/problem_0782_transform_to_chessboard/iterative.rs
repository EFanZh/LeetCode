pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn count_out_of_place(iter: impl Iterator<Item = i32>) -> usize {
        let mut expected = 0;
        let mut out_of_place = 0;

        for value in iter {
            if value != expected {
                out_of_place += 1;
            }

            expected = 1 - expected;
        }

        out_of_place
    }

    fn min_steps_helper(iter: impl Iterator<Item = i32>, n: usize) -> usize {
        let out_of_place = Self::count_out_of_place(iter);

        if n % 2 == 0 {
            out_of_place.min(n - out_of_place) / 2
        } else if out_of_place % 2 == 0 {
            out_of_place / 2
        } else {
            (n - out_of_place) / 2
        }
    }

    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let (first_row, rest_rows) = board.split_first().unwrap();
        let n = first_row.len();
        let half = n / 2;
        let is_half = |x| x == half || x == n - half;

        if is_half(first_row.iter().filter(|&&value| value == 0).count()) {
            let mut same_count = 1;

            for row in rest_rows {
                let mut iter = first_row.iter().zip(row).map(|(lhs, rhs)| lhs == rhs);
                let same = iter.next().unwrap();

                if iter.any(|value| value != same) {
                    return -1;
                }

                if same {
                    same_count += 1;
                }
            }

            if is_half(same_count) {
                let count_1 = Self::min_steps_helper(first_row.iter().copied(), n);
                let count_2 = Self::min_steps_helper(board.iter().map(|row| row[0]), n);

                (count_1 + count_2) as _
            } else {
                -1
            }
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        Self::moves_to_chessboard(board)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
