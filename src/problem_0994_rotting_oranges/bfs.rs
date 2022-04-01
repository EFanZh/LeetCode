pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut queue = VecDeque::new();
        let mut fresh = 0;

        for (y, row) in grid.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                match value {
                    1 => fresh += 1,
                    2 => queue.push_back((y, x)),
                    _ => {}
                }
            }
        }

        let mut result = 0;

        loop {
            for _ in 0..queue.len() {
                let (y, x) = queue.pop_front().unwrap();

                for (next_y, next_x) in [(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)] {
                    if let Some(value @ 1) = grid.get_mut(next_y).and_then(|row| row.get_mut(next_x)) {
                        *value = 2;

                        queue.push_back((next_y, next_x));

                        fresh -= 1;
                    }
                }
            }

            if queue.is_empty() {
                return if fresh == 0 { result } else { -1 };
            }

            result += 1;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        Self::oranges_rotting(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
