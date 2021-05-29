pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut rows = matrix.as_slice();
        let mut column_start = 0;
        let mut column_end = rows.first().map_or(0, Vec::len);

        while let Some((first_row, rest_rows)) = rows.split_first() {
            if let Some((last_row, rest_rows)) = rest_rows.split_last() {
                match (column_start + 1).cmp(&column_end) {
                    Ordering::Less => {
                        result.extend(&first_row[column_start..column_end]);
                        result.extend(rest_rows.iter().map(|row| row[column_end - 1]));
                        result.extend(last_row[column_start..column_end].iter().rev());
                        result.extend(rest_rows.iter().rev().map(|row| row[column_start]));

                        rows = rest_rows;
                        column_start += 1;
                        column_end -= 1;
                    }
                    Ordering::Equal => {
                        result.extend(rows.iter().map(|row| row[column_start]));

                        break;
                    }
                    Ordering::Greater => break,
                }
            } else {
                result.extend(&first_row[column_start..column_end]);

                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        Self::spiral_order(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
