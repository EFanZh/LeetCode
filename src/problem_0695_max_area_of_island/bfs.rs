pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    fn bfs(grid: &mut [Vec<i32>], mut i: usize, mut j: usize, queue: &mut VecDeque<(usize, usize)>) -> i32 {
        let mut result = 1;

        loop {
            for (offset_i, offset_j) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let next_i = i.wrapping_add(offset_i as _);
                let next_j = j.wrapping_add(offset_j as _);

                if let Some(value @ 1) = grid.get_mut(next_i).and_then(|row| row.get_mut(next_j)) {
                    *value = 2;

                    result += 1;

                    queue.push_back((next_i, next_j));
                }
            }

            if let Some((next_i, next_j)) = queue.pop_front() {
                i = next_i;
                j = next_j;
            } else {
                break;
            }
        }

        result
    }

    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut queue = VecDeque::new();
        let mut result = 0;

        for i in 0..rows {
            for j in 0..columns {
                if let value @ 1 = &mut grid[i][j] {
                    *value = 2;

                    result = result.max(Self::bfs(&mut grid, i, j, &mut queue));
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        Self::max_area_of_island(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
