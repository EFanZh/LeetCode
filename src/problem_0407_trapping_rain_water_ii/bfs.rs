pub struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct QueueItem {
    max_height: i32,
    row: usize,
    column: usize,
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.max_height.eq(&other.max_height)
    }
}

impl Eq for QueueItem {}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.max_height.cmp(&self.max_height)
    }
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let rows = height_map.len();
        let columns = height_map.first().map_or(0, Vec::len);

        if rows > 2 && columns > 2 {
            let mut queue = Vec::with_capacity((columns - 2) * (rows - 2));

            queue.extend(
                (1..)
                    .zip(&height_map[0][1..columns - 1])
                    .map(|(column, &max_height)| QueueItem {
                        max_height,
                        row: 0,
                        column,
                    }),
            );

            for (i, xs) in height_map.iter().enumerate().take(rows - 1).skip(1) {
                queue.push(QueueItem {
                    max_height: xs[0],
                    row: i,
                    column: 0,
                });

                queue.push(QueueItem {
                    max_height: xs[columns - 1],
                    row: i,
                    column: columns - 1,
                });
            }

            queue.extend(
                (1..)
                    .zip(&height_map[rows - 1][1..columns - 1])
                    .map(|(column, &max_height)| QueueItem {
                        max_height,
                        row: rows - 1,
                        column,
                    }),
            );

            let mut queue = BinaryHeap::from(queue);
            let mut visited = vec![false; columns * rows];

            visited[..columns].iter_mut().for_each(|x| *x = true);

            visited[columns..columns * (rows - 1)]
                .iter_mut()
                .step_by(columns)
                .for_each(|x| *x = true);

            visited[columns * 2 - 1..columns * (rows - 1)]
                .iter_mut()
                .step_by(columns)
                .for_each(|x| *x = true);

            visited[columns * (rows - 1)..].iter_mut().for_each(|x| *x = true);

            while let Some(QueueItem {
                max_height,
                row,
                column,
            }) = queue.pop()
            {
                for &(next_row, next_column) in &[
                    (row.wrapping_sub(1), column),
                    (row, column.wrapping_sub(1)),
                    (row, column + 1),
                    (row + 1, column),
                ] {
                    if let Some(&next_height) = height_map.get(next_row).and_then(|x| x.get(next_column)) {
                        if let state @ false = &mut visited[columns * next_row + next_column] {
                            *state = true;

                            if next_height < max_height {
                                result += max_height - next_height;

                                queue.push(QueueItem {
                                    max_height,
                                    row: next_row,
                                    column: next_column,
                                });
                            } else {
                                queue.push(QueueItem {
                                    max_height: next_height,
                                    row: next_row,
                                    column: next_column,
                                });
                            }
                        }
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        Self::trap_rain_water(height_map)
    }
}

#[cfg(test)]
mod tests {
    use super::QueueItem;

    #[test]
    fn test_queue_item() {
        assert!(
            QueueItem {
                max_height: 3,
                row: 5,
                column: 7
            } == QueueItem {
                max_height: 3,
                row: 11,
                column: 13
            }
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
