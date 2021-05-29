pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut rows = matrix.as_slice();
        let mut columns = rows.first().map_or(0, Vec::len);

        while let Some((first_row, rest_rows)) = rows.split_first() {
            if let Some(value) = first_row.get(columns.wrapping_sub(1)) {
                match target.cmp(value) {
                    Ordering::Less => columns -= 1,
                    Ordering::Equal => return true,
                    Ordering::Greater => rows = rest_rows,
                }
            } else {
                break;
            }
        }

        false
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
