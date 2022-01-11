pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let required = cols as u32 as usize * rows as u32 as usize;
        let mut result = Vec::with_capacity(required);
        let mut row = r_start;
        let mut column = c_start;
        let mut i = 1;

        loop {
            // Right.

            if row >= 0 {
                result.extend((column.max(0)..(column + i).min(cols)).map(|c| vec![row, c]));

                if result.len() == required {
                    break;
                }
            }

            column += i;

            // Down.

            if column < cols {
                result.extend((row.max(0)..(row + i).min(rows)).map(|r| vec![r, column]));

                if result.len() == required {
                    break;
                }
            }

            row += i;
            i += 1;

            // Left.

            if row < rows {
                result.extend(
                    ((column - i + 1).max(0)..(column + 1).min(cols))
                        .rev()
                        .map(|c| vec![row, c]),
                );

                if result.len() == required {
                    break;
                }
            }

            column -= i;

            // Up.

            if column >= 0 {
                result.extend(
                    ((row - i + 1).max(0)..(row + 1).min(rows))
                        .rev()
                        .map(|r| vec![r, column]),
                );

                if result.len() == required {
                    break;
                }
            }

            row -= i;
            i += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        Self::spiral_matrix_iii(rows, cols, r_start, c_start)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
