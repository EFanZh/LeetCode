pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    fn mark_first_island(grid: &mut [Vec<i32>]) -> VecDeque<(usize, usize)> {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut result = VecDeque::new();

        'outer: for y in 0..rows {
            for x in 0..columns {
                if let state @ 1 = &mut grid[y][x] {
                    *state = 2;

                    let mut position = (y, x);
                    let mut queue = VecDeque::new();

                    loop {
                        let (y, x) = position;

                        for &(next_y, next_x) in
                            &[(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)]
                        {
                            match grid.get_mut(next_y).and_then(|row| row.get_mut(next_x)) {
                                Some(state @ 0) => {
                                    *state = 3;

                                    result.push_back((next_y, next_x));
                                }
                                Some(state @ 1) => {
                                    *state = 2;

                                    queue.push_back((next_y, next_x));
                                }
                                _ => {}
                            }
                        }

                        if let Some(next_position) = queue.pop_front() {
                            position = next_position;
                        } else {
                            break 'outer;
                        }
                    }
                }
            }
        }

        result
    }

    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut queue = Self::mark_first_island(&mut grid);

        loop {
            let (y, x) = queue.pop_front().unwrap();
            let current_state = grid[y][x];

            for &(next_y, next_x) in &[(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)] {
                match grid.get_mut(next_y).and_then(|row| row.get_mut(next_x)) {
                    Some(state @ 0) => {
                        *state = current_state + 1;

                        queue.push_back((next_y, next_x));
                    }
                    Some(1) => return current_state - 2,
                    _ => {}
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        Self::shortest_bridge(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
