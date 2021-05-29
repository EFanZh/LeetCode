pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let modular = 1_000_000_007;
        let m = m as usize;
        let n = n as usize;
        let max_move = max_move as usize;
        let start_row = start_row as usize;
        let start_column = start_column as usize;
        let mut result = 0;
        let mut cache = vec![0_u32; n * m];
        let mut temp = vec![0_u32; n * m];

        for current_max_move in (0..max_move).rev() {
            let border = if current_max_move == max_move - 1 { 1 } else { 0 };

            for y in start_row.saturating_sub(current_max_move)..(start_row + current_max_move + 1).min(m) {
                let x_max_move = current_max_move - start_row.checked_sub(y).unwrap_or_else(|| y - start_row);

                for x in start_column.saturating_sub(x_max_move)..(start_column + x_max_move + 1).min(n) {
                    let top = if y == 0 { border } else { cache[n * (y - 1) + x] };
                    let left = if x == 0 { border } else { cache[n * y + (x - 1)] };
                    let right = if x == n - 1 { border } else { cache[n * y + (x + 1)] };
                    let bottom = if y == m - 1 { border } else { cache[n * (y + 1) + x] };

                    temp[n * y + x] = (top + left + right + bottom) % modular;
                }
            }

            result = (result + temp[n * start_row + start_column]) % modular;

            mem::swap(&mut cache, &mut temp);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        Self::find_paths(m, n, max_move, start_row, start_column)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
