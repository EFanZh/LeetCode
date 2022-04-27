pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut queue = VecDeque::new();
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);

        if rows < 3 || columns < 3 {
            0
        } else {
            let add_to_queue = |queue: &mut VecDeque<(usize, usize)>, y, x, value: &mut i32| {
                if *value != 0 {
                    *value = 0;
                    queue.push_back((y, x));
                }
            };

            for y in [0, rows - 1] {
                for (x, value) in grid[y].iter_mut().enumerate() {
                    add_to_queue(&mut queue, y, x, value);
                }
            }

            for (y, row) in (1..).zip(&mut grid[1..rows - 1]) {
                for x in [0, row.len() - 1] {
                    add_to_queue(&mut queue, y, x, &mut row[x]);
                }
            }

            while let Some((y, x)) = queue.pop_front() {
                for (next_y, next_x) in [(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)] {
                    if let Some(value) = grid.get_mut(next_y).and_then(|row| row.get_mut(next_x)) {
                        add_to_queue(&mut queue, next_y, next_x, value);
                    }
                }
            }

            let mut result = 0;

            for row in &grid {
                for value in row {
                    result += value;
                }
            }

            result
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        Self::num_enclaves(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
