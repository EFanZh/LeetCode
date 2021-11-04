pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut group = 2;
        let mut queue = VecDeque::new();
        let mut group_sizes = Vec::new();

        for y in 0..rows {
            for x in 0..columns {
                if let state @ 1 = &mut grid[y][x] {
                    *state = group;

                    let mut y = y;
                    let mut x = x;
                    let mut count = 1;

                    loop {
                        for &(next_y, next_x) in
                            &[(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)]
                        {
                            if let Some(state @ 1) = grid.get_mut(next_y).and_then(|row| row.get_mut(next_x)) {
                                *state = group;
                                count += 1;

                                queue.push_back((next_y, next_x));
                            }
                        }

                        if let Some((next_y, next_x)) = queue.pop_front() {
                            y = next_y;
                            x = next_x;
                        } else {
                            break;
                        }
                    }

                    group_sizes.push(count);
                    group += 1;
                }
            }
        }

        match group_sizes.as_slice() {
            [] => 1,
            &[size] => (size + 1).min((columns * rows) as _),
            group_sizes => {
                let mut buffer = [0; 4];
                let mut result = 0;

                for y in 0..rows {
                    for x in 0..columns {
                        if grid[y][x] == 0 {
                            let mut buffer_length = 0;

                            for &(next_y, next_x) in
                                &[(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)]
                            {
                                if let Some(&group) = grid.get(next_y).and_then(|row| row.get(next_x)) {
                                    if group != 0 && !buffer[..buffer_length].contains(&group) {
                                        buffer[buffer_length] = group;
                                        buffer_length += 1;
                                    }
                                }
                            }

                            result = result.max(
                                buffer[..buffer_length]
                                    .iter()
                                    .map(|&group| group_sizes[group as usize - 2])
                                    .sum::<i32>()
                                    + 1,
                            );
                        }
                    }
                }

                result
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        Self::largest_island(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
