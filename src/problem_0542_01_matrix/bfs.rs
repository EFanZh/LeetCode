pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;

        let mut queue = mat
            .iter_mut()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter_mut().enumerate().filter_map(move |(j, value)| {
                    if *value == 0 {
                        Some((i, j))
                    } else {
                        *value = -1;

                        None
                    }
                })
            })
            .collect::<VecDeque<_>>();

        let mut depth = 1;

        loop {
            for _ in 0..queue.len() {
                let (row, column) = queue.pop_front().unwrap();

                for &(next_row, next_column) in &[
                    (row.wrapping_sub(1), column),
                    (row, column.wrapping_sub(1)),
                    (row, column + 1),
                    (row + 1, column),
                ] {
                    if let Some(slot @ -1) = mat.get_mut(next_row).and_then(|row| row.get_mut(next_column)) {
                        *slot = depth;
                        queue.push_back((next_row, next_column));
                    }
                }
            }

            if queue.is_empty() {
                break;
            }

            depth += 1;
        }

        mat
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::update_matrix(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
