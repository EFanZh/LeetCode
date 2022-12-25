pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let target = (grid.len() - 1, grid.first().map_or(0, Vec::len) - 1);
        let mut result = 0;
        let mut queue = VecDeque::new();
        let mut current = (0, 0);

        while let Some(direction) = grid.get_mut(current.0).and_then(|row| row.get_mut(current.1)) {
            if current == target {
                return result;
            }

            queue.push_back(current);

            current = match mem::replace(direction, 0) {
                1 => (current.0, current.1 + 1),
                2 => (current.0, current.1.wrapping_sub(1)),
                3 => (current.0 + 1, current.1),
                _ => (current.0.wrapping_sub(1), current.1),
            };
        }

        loop {
            result += 1;

            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();

                for mut neighbor in [
                    (current.0.wrapping_sub(1), current.1),
                    (current.0, current.1.wrapping_sub(1)),
                    (current.0, current.1 + 1),
                    (current.0 + 1, current.1),
                ] {
                    while let Some(state) = grid
                        .get_mut(neighbor.0)
                        .and_then(|row| row.get_mut(neighbor.1))
                        .filter(|state| **state != 0)
                    {
                        if neighbor == target {
                            return result;
                        }

                        queue.push_back(neighbor);

                        neighbor = match mem::replace(state, 0) {
                            1 => (neighbor.0, neighbor.1 + 1),
                            2 => (neighbor.0, neighbor.1.wrapping_sub(1)),
                            3 => (neighbor.0 + 1, neighbor.1),
                            _ => (neighbor.0.wrapping_sub(1), neighbor.1),
                        };
                    }
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        Self::min_cost(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
