pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::vec::IntoIter;

impl Solution {
    fn has_row(first_row: Vec<i32>, rest_rows: IntoIter<Vec<i32>>, column: usize, target: i32) -> bool {
        if let Some(&value) = first_row.get(column) {
            Self::dispatch(first_row, rest_rows, column, target, value)
        } else {
            false
        }
    }

    fn has_columns(mut rows: IntoIter<Vec<i32>>, column: usize, target: i32) -> bool {
        rows.next().map_or(false, |first_row| {
            let value = first_row[column];

            Self::dispatch(first_row, rows, column, target, value)
        })
    }

    fn dispatch(first_row: Vec<i32>, rest_rows: IntoIter<Vec<i32>>, column: usize, target: i32, value: i32) -> bool {
        match target.cmp(&value) {
            Ordering::Less => Self::has_row(first_row, rest_rows, column.wrapping_sub(1), target),
            Ordering::Equal => true,
            Ordering::Greater => Self::has_columns(rest_rows, column, target),
        }
    }

    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        matrix
            .first()
            .map_or(0, Vec::len)
            .checked_sub(1)
            .map_or(false, |column| Self::has_columns(matrix.into_iter(), column, target))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        Self::search_matrix(matrix, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
