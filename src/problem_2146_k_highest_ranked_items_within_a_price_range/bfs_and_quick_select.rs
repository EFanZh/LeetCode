pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;
use std::mem;

impl Solution {
    pub fn highest_ranked_k_items(grid: Vec<Vec<i32>>, pricing: Vec<i32>, start: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let [low, high] = pricing.try_into().ok().unwrap();
        let [start_row, start_col] = start.try_into().ok().unwrap();
        let price_range = low as u32..=high as u32;
        let k = k as u32 as usize;
        let node = (start_row as u32, start_col as u32);
        let mut distance = 1;
        let mut queue = VecDeque::from([node]);
        let mut candidates = Vec::<(u32, u32, u32, u32)>::with_capacity(columns * rows);
        let start_price = mem::take(&mut grid[node.0 as usize][node.1 as usize]) as _;

        if price_range.contains(&start_price) {
            candidates.push((0, start_price, node.0, node.1));
        }

        loop {
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();

                for next in [
                    (node.0.wrapping_sub(1), node.1),
                    (node.0, node.1.wrapping_sub(1)),
                    (node.0, node.1 + 1),
                    (node.0 + 1, node.1),
                ] {
                    if let Some(cell) = grid
                        .get_mut(next.0 as usize)
                        .and_then(|row| row.get_mut(next.1 as usize))
                    {
                        if *cell != 0 {
                            let price = mem::take(cell) as _;

                            if price_range.contains(&price) {
                                candidates.push((distance, price, next.0, next.1));
                            }

                            queue.push_back(next);
                        }
                    }
                }
            }

            if queue.is_empty() {
                break;
            }

            distance += 1;
        }

        if k < candidates.len() {
            candidates.select_nth_unstable(k);
            candidates.truncate(k);
        }

        candidates.sort_unstable();

        candidates
            .into_iter()
            .map(|(_, _, row, column)| vec![row as _, column as _])
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn highest_ranked_k_items(grid: Vec<Vec<i32>>, pricing: Vec<i32>, start: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        Self::highest_ranked_k_items(grid, pricing, start, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
