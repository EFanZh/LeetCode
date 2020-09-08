pub struct Solution;

impl Solution {
    fn total_n_queens_helper(
        n: usize,
        row: usize,
        column_hits: &mut [bool],
        main_diagonal_hits: &mut [bool],
        anti_diagonal_hits: &mut [bool],
        result: &mut i32,
    ) {
        if row == n {
            *result += 1;
        } else {
            for column in 0..n {
                let main_diagonal = n - 1 - row + column;
                let anti_diagonal = row + column;

                if !(column_hits[column] || main_diagonal_hits[main_diagonal] || anti_diagonal_hits[anti_diagonal]) {
                    column_hits[column] = true;
                    main_diagonal_hits[main_diagonal] = true;
                    anti_diagonal_hits[anti_diagonal] = true;

                    Self::total_n_queens_helper(
                        n,
                        row + 1,
                        column_hits,
                        main_diagonal_hits,
                        anti_diagonal_hits,
                        result,
                    );

                    column_hits[column] = false;
                    main_diagonal_hits[main_diagonal] = false;
                    anti_diagonal_hits[anti_diagonal] = false;
                }
            }
        }
    }

    pub fn total_n_queens(n: i32) -> i32 {
        if n < 2 {
            1
        } else {
            let n = n as usize;
            let mut result = 0;

            Self::total_n_queens_helper(
                n,
                0,
                &mut vec![false; n],
                &mut vec![false; n * 2 - 1],
                &mut vec![false; n * 2 - 1],
                &mut result,
            );

            result
        }
    }
}

impl super::Solution for Solution {
    fn total_n_queens(n: i32) -> i32 {
        Self::total_n_queens(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
