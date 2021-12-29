pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let mut start_row = 0;
        let mut start_column = 0;
        let mut remaining = 0;

        for (row, row_data) in (0_u8..).zip(&grid) {
            for (column, cell) in (0_u8..).zip(row_data.bytes()) {
                match cell {
                    b'@' => {
                        start_row = row;
                        start_column = column;
                    }
                    b'A'..=b'Z' => remaining |= 1 << (cell - b'A'),
                    _ => {}
                }
            }
        }

        let mut queue = VecDeque::from(vec![(start_row, start_column, remaining)]);

        let mut visited = Some((start_row, start_column, remaining))
            .into_iter()
            .collect::<HashSet<_>>();

        let mut result = 1;

        loop {
            for _ in 0..queue.len() {
                let (row, column, remaining) = queue.pop_front().unwrap();

                for &(next_row, next_column) in &[
                    (row.wrapping_sub(1), column),
                    (row, column.wrapping_sub(1)),
                    (row, column + 1),
                    (row + 1, column),
                ] {
                    if let Some(&next_cell) = grid
                        .get(usize::from(next_row))
                        .and_then(|next_row| next_row.as_bytes().get(usize::from(next_column)))
                    {
                        let next_remaining = match next_cell {
                            b'.' | b'@' => remaining,
                            b'#' => continue,
                            b'A'..=b'Z' => {
                                if remaining & (1_u8 << (next_cell - b'A')) == 0 {
                                    remaining
                                } else {
                                    continue;
                                }
                            }
                            _ => {
                                let next_remaining = remaining & !(1_u8 << (next_cell - b'a'));

                                if next_remaining == 0 {
                                    return result;
                                }

                                next_remaining
                            }
                        };

                        let next = (next_row, next_column, next_remaining);

                        if visited.insert(next) {
                            queue.push_back(next);
                        }
                    }
                }
            }

            if queue.is_empty() {
                return -1;
            }

            result += 1;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        Self::shortest_path_all_keys(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
