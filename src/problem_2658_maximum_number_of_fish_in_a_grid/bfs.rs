pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;

impl Solution {
    pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut result = 0_u32;
        let mut queue = VecDeque::new();

        for row in 0..rows {
            for column in 0..columns {
                let state = &mut grid[row][column];

                if *state != 0 {
                    let mut count = mem::take(state) as u32;
                    let mut row = row;
                    let mut column = column;

                    loop {
                        for (next_row, next_column) in [
                            (row.wrapping_sub(1), column),
                            (row, column.wrapping_sub(1)),
                            (row, column + 1),
                            (row + 1, column),
                        ] {
                            if let Some(next_state) = grid.get_mut(next_row).and_then(|row| row.get_mut(next_column))
                                && *next_state != 0
                            {
                                count += mem::take(next_state) as u32;

                                queue.push_back(((next_row as u8) << 4) | next_column as u8);
                            }
                        }

                        if let Some(next) = queue.pop_front() {
                            row = usize::from(next >> 4);
                            column = usize::from(next & 0b_1111);
                        } else {
                            break;
                        }
                    }

                    result = result.max(count);
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
        Self::find_max_fish(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
