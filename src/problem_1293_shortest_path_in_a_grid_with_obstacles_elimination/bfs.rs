pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);

        if rows == 1 && columns == 1 {
            return 0;
        }

        let mut max_budgets = vec![-1; columns * rows];

        max_budgets[0] = k;

        let mut queue = VecDeque::<(usize, usize, i32)>::from([(0, 0, k)]);
        let mut result = 1;

        loop {
            for _ in 0..queue.len() {
                let (y, x, budget) = queue.pop_front().unwrap();

                for (next_y, next_x) in [(y.wrapping_sub(1), x), (y, x.wrapping_sub(1)), (y, x + 1), (y + 1, x)] {
                    if next_y == rows - 1 && next_x == columns - 1 {
                        return result;
                    }

                    if let Some(&next_state) = grid.get(next_y).and_then(|row| row.get(next_x))
                        && let Some(next_budget) = u32::checked_sub(budget as _, next_state as _)
                    {
                        let next_budget = next_budget as i32;
                        let max_budget = &mut max_budgets[columns * next_y + next_x];

                        if next_budget > *max_budget {
                            *max_budget = next_budget;

                            queue.push_back((next_y, next_x, next_budget));
                        }
                    }
                }
            }

            if queue.is_empty() {
                return -1;
            }

            result += 1;
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::shortest_path(grid, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
