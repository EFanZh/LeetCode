pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let n = grid.len();
        let mut queue = VecDeque::new();

        for (y, row) in grid.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value != 0 {
                    queue.push_back((y, x));
                }
            }
        }

        if queue.is_empty() || queue.len() == n * n {
            -1
        } else {
            let mut result = 0;

            loop {
                for _ in 0..queue.len() {
                    let (y, x) = queue.pop_front().unwrap();

                    for (next_y, next_x) in [(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)] {
                        if let Some(value @ 0) = grid.get_mut(next_y).and_then(|row| row.get_mut(next_x)) {
                            *value = 1;

                            queue.push_back((next_y, next_x));
                        }
                    }
                }

                if queue.is_empty() {
                    break;
                }

                result += 1;
            }

            result
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        Self::max_distance(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
