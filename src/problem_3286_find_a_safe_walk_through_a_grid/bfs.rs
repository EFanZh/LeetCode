pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let mut grid = grid;
        let mut budget = health - mem::replace(&mut grid[0][0], 2);
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let target = (rows - 1, columns - 1);

        if budget == 0 {
            return false;
        } else if target == (0, 0) {
            return true;
        }

        let mut current = vec![(0_u8, 0_u8)];
        let mut next = Vec::new();

        loop {
            while let Some(node) = current.pop() {
                let node = (usize::from(node.0), usize::from(node.1));

                for neighbor in [
                    (node.0.wrapping_sub(1), node.1),
                    (node.0, node.1.wrapping_sub(1)),
                    (node.0, node.1 + 1),
                    (node.0 + 1, node.1),
                ] {
                    if let Some(state @ (0 | 1)) = grid.get_mut(neighbor.0).and_then(|row| row.get_mut(neighbor.1)) {
                        if neighbor == target && budget - *state != 0 {
                            return true;
                        }

                        (if mem::replace(state, 2) == 0 {
                            &mut current
                        } else {
                            &mut next
                        })
                        .push((neighbor.0 as _, neighbor.1 as _));
                    }
                }
            }

            budget -= 1;

            if budget == 0 {
                return false;
            }

            mem::swap(&mut current, &mut next);
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        Self::find_safe_walk(grid, health)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
