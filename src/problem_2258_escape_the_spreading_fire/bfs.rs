pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

struct Distances {
    top: u16,
    left: u16,
    center: u16,
    required: u8,
}

impl Distances {
    fn update(&mut self, rows: u16, columns: u16, steps: u16, node: (u16, u16)) -> bool {
        if node.0 == rows - 2 {
            if node.1 == columns - 1 {
                self.top = steps;
            } else {
                return false;
            }
        } else if node.0 == rows - 1 {
            if node.1 == columns - 2 {
                self.left = steps;
            } else if node.1 == columns - 1 {
                self.center = steps;
            } else {
                return false;
            }
        } else {
            return false;
        };

        self.required -= 1;

        self.required == 0
    }
}

impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut queue = VecDeque::from([(0_u16, 0_u16)]);

        let mut start_distances = Distances {
            top: u16::MAX,
            left: u16::MAX,
            center: u16::MAX,
            required: 1 + u8::from(grid[rows - 2][columns - 1] == 0) + u8::from(grid[rows - 1][columns - 2] == 0),
        };

        let mut steps = 1;

        grid[0][0] = 3;

        'outer: loop {
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                for neighbor in [
                    (node.0.wrapping_sub(1), node.1),
                    (node.0, node.1.wrapping_sub(1)),
                    (node.0, node.1 + 1),
                    (node.0 + 1, node.1),
                ] {
                    if let Some(state) = grid
                        .get_mut(usize::from(neighbor.0))
                        .and_then(|row| row.get_mut(usize::from(neighbor.1)))
                    {
                        if *state == 0 {
                            if start_distances.update(rows as _, columns as _, steps, neighbor) {
                                break 'outer;
                            }

                            *state = 3;

                            queue.push_back(neighbor);
                        }
                    }
                }
            }

            if queue.is_empty() {
                return -1;
            }

            steps += 1;
        }

        queue.clear();

        (0..).zip(&grid).for_each(|(y, row)| {
            queue.extend((0..).zip(row).filter_map(|(x, &value)| (value == 1).then_some((y, x))));
        });

        let top_state = grid[rows - 2][columns - 1];
        let left_state = grid[rows - 1][columns - 2];

        let mut fire_distances = Distances {
            top: if top_state == 1 { 0 } else { u16::MAX },
            left: if left_state == 1 { 0 } else { u16::MAX },
            center: u16::MAX,
            required: 1 + u8::from(matches!(top_state, 0 | 3)) + u8::from(matches!(left_state, 0 | 3)),
        };

        let mut steps = 1;

        'outer: loop {
            if queue.is_empty() {
                return 1_000_000_000;
            }

            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                for neighbor in [
                    (node.0.wrapping_sub(1), node.1),
                    (node.0, node.1.wrapping_sub(1)),
                    (node.0, node.1 + 1),
                    (node.0 + 1, node.1),
                ] {
                    if let Some(state) = grid
                        .get_mut(usize::from(neighbor.0))
                        .and_then(|row| row.get_mut(usize::from(neighbor.1)))
                    {
                        if matches!(state, 0 | 3) {
                            if fire_distances.update(rows as _, columns as _, steps, neighbor) {
                                break 'outer;
                            }

                            *state = 1;

                            queue.push_back(neighbor);
                        }
                    }
                }
            }

            steps += 1;
        }

        if fire_distances.center < start_distances.center {
            return -1;
        }

        let top_diff = i32::from(fire_distances.top) - i32::from(start_distances.top);
        let left_diff = i32::from(fire_distances.left) - i32::from(start_distances.left);
        let center_diff = i32::from(fire_distances.center) - i32::from(start_distances.center);

        (top_diff.max(left_diff) - 1).min(center_diff)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        Self::maximum_minutes(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
