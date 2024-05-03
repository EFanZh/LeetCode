pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut queue = VecDeque::new();
        let mut result = 0;

        for y in 0..rows {
            for x in 0..columns {
                if let state @ 0 = &mut grid[y][x] {
                    *state = 1;

                    let mut node = (y, x);
                    let mut on_border = false;

                    loop {
                        for next in [
                            (node.0.wrapping_sub(1), node.1),
                            (node.0, node.1.wrapping_sub(1)),
                            (node.0, node.1 + 1),
                            (node.0 + 1, node.1),
                        ] {
                            if let Some(next_state) = grid.get_mut(next.0).and_then(|row| row.get_mut(next.1)) {
                                if *next_state == 0 {
                                    *next_state = 1;

                                    queue.push_back(next);
                                }
                            } else {
                                on_border = true;
                            }
                        }

                        if let Some(next) = queue.pop_front() {
                            node = next;
                        } else {
                            break;
                        }
                    }

                    if !on_border {
                        result += 1;
                    }
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        Self::closed_island(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
