pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn helper(grid: &mut [Vec<i32>], y: usize, x: usize, mut gold: u32, result: &mut u32) {
        if let Some(value) = grid
            .get_mut(y)
            .and_then(|row| row.get_mut(x))
            .filter(|value| **value != 0)
        {
            let saved_value = mem::replace(value, 0);

            gold += saved_value as u32;

            Self::helper(grid, y.wrapping_sub(1), x, gold, result);
            Self::helper(grid, y, x.wrapping_sub(1), gold, result);
            Self::helper(grid, y, x + 1, gold, result);
            Self::helper(grid, y + 1, x, gold, result);

            grid[y][x] = saved_value;
        } else {
            *result = (*result).max(gold);
        }
    }

    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut result = 0;

        for y in 0..rows {
            for x in 0..columns {
                Self::helper(&mut grid, y, x, 0, &mut result);
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        Self::get_maximum_gold(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
