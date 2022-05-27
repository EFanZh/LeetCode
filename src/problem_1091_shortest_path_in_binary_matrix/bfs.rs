pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let n = grid.len();
        let mut length = 1;

        if let value @ 0 = &mut grid[0][0] {
            if n == 1 {
                return length;
            }

            *value = 1;

            let mut queue = VecDeque::from([(0_usize, 0_usize)]);

            loop {
                length += 1;

                for _ in 0..queue.len() {
                    let node = queue.pop_front().unwrap();

                    for next in [
                        (node.0.wrapping_sub(1), node.1.wrapping_sub(1)),
                        (node.0.wrapping_sub(1), node.1),
                        (node.0.wrapping_sub(1), node.1 + 1),
                        (node.0, node.1.wrapping_sub(1)),
                        (node.0, node.1 + 1),
                        (node.0 + 1, node.1.wrapping_sub(1)),
                        (node.0 + 1, node.1),
                        (node.0 + 1, node.1 + 1),
                    ] {
                        if let Some(value @ 0) = grid.get_mut(next.0).and_then(|row| row.get_mut(next.1)) {
                            if next == (n - 1, n - 1) {
                                return length;
                            }

                            *value = 1;

                            queue.push_back(next);
                        }
                    }
                }

                if queue.is_empty() {
                    break;
                }
            }
        }

        -1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        Self::shortest_path_binary_matrix(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
