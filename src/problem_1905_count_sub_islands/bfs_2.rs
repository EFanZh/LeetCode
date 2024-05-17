pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let rows = grid1.len();
        let columns = grid1.first().map_or(0, Vec::len);

        assert!(rows <= 500);
        assert!(columns <= 500);

        let mut grid = vec![0_u8; columns * rows].into_boxed_slice();
        let mut grid_writer = grid.iter_mut();

        for (row_1, row_2) in grid1.into_iter().zip(grid2) {
            for ((value_1, value_2), target) in row_1.into_iter().zip(row_2).zip(&mut grid_writer) {
                *target = ((value_1 as u8) << 1) | value_2 as u8;
            }
        }

        let mut result = 0;
        let mut queue = VecDeque::new();

        for y in 0..rows {
            for x in 0..columns {
                let state = &mut grid[columns * y + x];

                if *state & 1 != 0 {
                    let mut is_sub_island = (*state >> 1) != 0;

                    *state = 0;

                    let mut y = y;
                    let mut x = x;

                    loop {
                        for next in [(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)] {
                            if next.1 < columns {
                                let next_index = columns.wrapping_mul(next.0).wrapping_add(next.1);

                                if let Some(state) = grid.get_mut(next_index) {
                                    if *state & 1 != 0 {
                                        is_sub_island &= (*state >> 1) != 0;

                                        *state = 0;

                                        queue.push_back((next.0 as u16, next.1 as u16));
                                    }
                                }
                            }
                        }

                        if let Some(next) = queue.pop_front() {
                            y = usize::from(next.0);
                            x = usize::from(next.1);
                        } else {
                            break;
                        }
                    }

                    result += i32::from(is_sub_island);
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        Self::count_sub_islands(grid1, grid2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
