pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let target = (rows as i32 - 1, columns as i32 - 1);
        let mut distance = 0;
        let mut queue_1 = VecDeque::new();
        let mut queue_2 = VecDeque::new();
        let mut node = (0_i32, 0_i32);

        loop {
            for neighbor in [
                (node.0.wrapping_sub(1), node.1),
                (node.0, node.1.wrapping_sub(1)),
                (node.0, node.1 + 1),
                (node.0 + 1, node.1),
            ] {
                if neighbor == target {
                    return distance;
                }

                if let Some(state) = grid
                    .get_mut(neighbor.0 as usize)
                    .and_then(|row| row.get_mut(neighbor.1 as usize))
                {
                    let queue = match *state {
                        0 => &mut queue_1,
                        1 => &mut queue_2,
                        _ => continue,
                    };

                    *state = 2;
                    queue.push_back((neighbor.0, neighbor.1));
                }
            }

            if queue_1.is_empty() {
                mem::swap(&mut queue_1, &mut queue_2);
                distance += 1;
            }

            node = queue_1.pop_front().unwrap();
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        Self::minimum_obstacles(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
