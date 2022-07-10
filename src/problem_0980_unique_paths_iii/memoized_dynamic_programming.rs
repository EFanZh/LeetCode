pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::num::NonZeroU8;

impl Solution {
    fn helper(state: u32, rows: u8, columns: NonZeroU8, end: u8, cache: &mut HashMap<u32, u32>) -> u32 {
        if let Some(&result) = cache.get(&state) {
            result
        } else {
            let start = (state >> 24) as u8;

            if start == end {
                if (state & ((1 << 24) - 1)).count_ones() as u8 == columns.get() * rows {
                    1
                } else {
                    0
                }
            } else {
                let start_row = start / columns;
                let start_column = start % columns;
                let mut result = 0;

                for (valid, next) in [
                    (start_row != 0, start.wrapping_sub(columns.get())),
                    (start_column != 0, start.wrapping_sub(1)),
                    (start_column != columns.get() - 1, start + 1),
                    (start_row != rows - 1, start + columns.get()),
                ] {
                    if valid && ((state >> next) & 1) == 0 {
                        let occupied = (state | (1 << next)) & ((1 << 24) - 1);

                        result += Self::helper(occupied | (u32::from(next) << 24), rows, columns, end, cache);
                    }
                }

                cache.insert(state, result);

                result
            }
        }
    }

    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut occupied = 0;
        let mut start = 0;
        let mut end = 0;
        let mut i = 0_u8;

        for row in &grid {
            for &value in row {
                match value {
                    -1 => occupied |= 1 << i,
                    0 => {}
                    1 => {
                        occupied |= 1 << i;
                        start = i;
                    }
                    _ => end = i,
                }

                i += 1;
            }
        }

        Self::helper(
            occupied | (u32::from(start) << 24),
            grid.len() as _,
            NonZeroU8::new(grid.first().map_or(0, Vec::len) as u8).unwrap(),
            end,
            &mut HashMap::new(),
        ) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        Self::unique_paths_iii(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
