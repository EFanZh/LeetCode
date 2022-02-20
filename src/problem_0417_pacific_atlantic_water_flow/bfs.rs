pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if !matrix.is_empty() {
            let rows = matrix.len();
            let columns = matrix.first().map_or(0, Vec::len);
            let mut states = vec![0_u8; columns * rows];
            let mut queue = VecDeque::new();

            for (mut row, mut column, mut current) in matrix[0]
                .iter()
                .enumerate()
                .map(|(i, &x)| (0, i, x))
                .chain(matrix.iter().enumerate().skip(1).map(|(i, xs)| (i, 0, xs[0])))
            {
                if let state @ 0 = &mut states[columns * row + column] {
                    *state = 1;

                    loop {
                        for (next_row, next_column) in [
                            (row.wrapping_sub(1), column),
                            (row, column.wrapping_sub(1)),
                            (row, column + 1),
                            (row + 1, column),
                        ] {
                            if let Some(&next) = matrix.get(next_row).and_then(|x| x.get(next_column)) {
                                if next >= current {
                                    if let next_state @ 0 = &mut states[columns * next_row + next_column] {
                                        *next_state = 1;

                                        queue.push_back((next_row, next_column, next));
                                    }
                                }
                            }
                        }

                        if let Some((next_row, next_column, next)) = queue.pop_front() {
                            row = next_row;
                            column = next_column;
                            current = next;
                        } else {
                            break;
                        }
                    }
                }
            }

            for (mut row, mut column, mut current) in matrix[..rows - 1]
                .iter()
                .enumerate()
                .map(|(i, xs)| (i, columns - 1, xs[columns - 1]))
                .chain(
                    matrix
                        .last()
                        .unwrap()
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| (rows - 1, i, x)),
                )
            {
                let state = &mut states[columns * row + column];

                if *state != 2 {
                    if *state == 1 {
                        result.push(vec![row as _, column as _]);
                    }

                    *state = 2;

                    loop {
                        for (next_row, next_column) in [
                            (row.wrapping_sub(1), column),
                            (row, column.wrapping_sub(1)),
                            (row, column + 1),
                            (row + 1, column),
                        ] {
                            if let Some(&next) = matrix.get(next_row).and_then(|x| x.get(next_column)) {
                                if next >= current {
                                    let next_state = &mut states[columns * next_row + next_column];

                                    if *next_state != 2 {
                                        if *next_state == 1 {
                                            result.push(vec![next_row as _, next_column as _]);
                                        }

                                        *next_state = 2;

                                        queue.push_back((next_row, next_column, next));
                                    }
                                }
                            }
                        }

                        if let Some((next_row, next_column, next)) = queue.pop_front() {
                            row = next_row;
                            column = next_column;
                            current = next;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::pacific_atlantic(matrix)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
