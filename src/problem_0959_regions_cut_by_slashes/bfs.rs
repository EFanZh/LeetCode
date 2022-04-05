pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

struct Neighbor {
    position: (usize, usize),
    grid_position: (usize, usize),
    separator: u8,
}

impl Solution {
    fn neighbors((y, x): (usize, usize)) -> [Neighbor; 4] {
        if y % 2 == 0 {
            [
                Neighbor {
                    position: (y.wrapping_sub(1), x),
                    grid_position: ((y / 2).wrapping_sub(1), x),
                    separator: b'/',
                },
                Neighbor {
                    position: (y.wrapping_sub(1), x + 1),
                    grid_position: ((y / 2).wrapping_sub(1), x),
                    separator: b'\\',
                },
                Neighbor {
                    position: (y + 1, x),
                    grid_position: (y / 2, x),
                    separator: b'\\',
                },
                Neighbor {
                    position: (y + 1, x + 1),
                    grid_position: (y / 2, x),
                    separator: b'/',
                },
            ]
        } else {
            [
                Neighbor {
                    position: (y.wrapping_sub(1), x.wrapping_sub(1)),
                    grid_position: (y / 2, x.wrapping_sub(1)),
                    separator: b'/',
                },
                Neighbor {
                    position: (y.wrapping_sub(1), x),
                    grid_position: (y / 2, x),
                    separator: b'\\',
                },
                Neighbor {
                    position: (y + 1, x.wrapping_sub(1)),
                    grid_position: (y / 2, x.wrapping_sub(1)),
                    separator: b'\\',
                },
                Neighbor {
                    position: (y + 1, x),
                    grid_position: (y / 2, x),
                    separator: b'/',
                },
            ]
        }
    }

    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let columns = n + 1;
        let mut visited = vec![false; columns * (n * 2 + 1)];
        let mut queue = VecDeque::new();
        let mut result = 0;

        for y in 0..=n * 2 {
            for x in 0..n + y % 2 {
                if let value @ false = &mut visited[columns * y + x] {
                    *value = true;
                    result += 1;

                    let mut node = (y, x);

                    loop {
                        for Neighbor {
                            position,
                            grid_position,
                            separator,
                        } in Self::neighbors(node)
                        {
                            if grid.get(grid_position.0).map_or(false, |row| {
                                row.as_bytes()
                                    .get(grid_position.1)
                                    .map_or(false, |&cell| cell != separator)
                            }) {
                                if let value @ false = &mut visited[columns * position.0 + position.1] {
                                    *value = true;

                                    queue.push_back(position);
                                }
                            }
                        }

                        if let Some(next_node) = queue.pop_front() {
                            node = next_node;
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
    fn regions_by_slashes(grid: Vec<String>) -> i32 {
        Self::regions_by_slashes(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
