pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;

impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let mut queue = VecDeque::new();
        let mut node = (row as usize, col as usize);
        let old_color = mem::replace(&mut grid[node.0][node.1], -1);

        loop {
            let mut is_border = false;

            for (next_y, next_x) in [
                (node.0.wrapping_sub(1), node.1),
                (node.0, node.1.wrapping_sub(1)),
                (node.0, node.1 + 1),
                (node.0 + 1, node.1),
            ] {
                if let Some(next_color) = grid.get_mut(next_y).and_then(|row| row.get_mut(next_x)) {
                    match next_color {
                        -2 | -1 => {}
                        color => {
                            if *color == old_color {
                                *color = -1;
                                queue.push_back((next_y, next_x));
                            } else {
                                is_border = true;
                            }
                        }
                    }
                } else {
                    is_border = true;
                }
            }

            if is_border {
                grid[node.0][node.1] = -2;
            }

            if let Some(next_node) = queue.pop_front() {
                node = next_node;
            } else {
                break;
            }
        }

        for row in &mut grid {
            for value in row {
                match value {
                    -2 => *value = color,
                    -1 => *value = old_color,
                    _ => {}
                }
            }
        }

        grid
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        Self::color_border(grid, row, col, color)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
