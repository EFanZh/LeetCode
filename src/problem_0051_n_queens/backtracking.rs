pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn solve_n_queens_helper(
        n: usize,
        row: usize,
        base: &mut [u8],
        column_hits: &mut [bool],
        main_diagonal_hits: &mut [bool],
        anti_diagonal_hits: &mut [bool],
        result: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            result.push(
                base.chunks_exact(n)
                    .map(|row| String::from_utf8(row.to_vec()).unwrap())
                    .collect(),
            );
        } else {
            for column in 0..n {
                let main_diagonal = n - 1 - row + column;
                let anti_diagonal = row + column;

                if !(column_hits[column] || main_diagonal_hits[main_diagonal] || anti_diagonal_hits[anti_diagonal]) {
                    base[n * row + column] = b'Q';
                    column_hits[column] = true;
                    main_diagonal_hits[main_diagonal] = true;
                    anti_diagonal_hits[anti_diagonal] = true;

                    Self::solve_n_queens_helper(
                        n,
                        row + 1,
                        base,
                        column_hits,
                        main_diagonal_hits,
                        anti_diagonal_hits,
                        result,
                    );

                    base[n * row + column] = b'.';
                    column_hits[column] = false;
                    main_diagonal_hits[main_diagonal] = false;
                    anti_diagonal_hits[anti_diagonal] = false;
                }
            }
        }
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut result = Vec::new();

        Self::solve_n_queens_helper(
            n,
            0,
            &mut vec![b'.'; n * n],
            &mut vec![false; n],
            &mut vec![false; n * 2 - 1],
            &mut vec![false; n * 2 - 1],
            &mut result,
        );

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        Self::solve_n_queens(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
