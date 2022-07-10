pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU8;

impl Solution {
    fn helper(state: u32, rows: u8, columns: NonZeroU8, start: u8, end: u8, result: &mut i32) {
        if start == end {
            if state == (1 << (columns.get() * rows)) - 1 {
                *result += 1;
            }
        } else {
            let y = start / columns;
            let x = start % columns;

            for (valid, next) in [
                (y != 0, start.wrapping_sub(columns.get())),
                (x != 0, start.wrapping_sub(1)),
                (x != columns.get() - 1, start + 1),
                (y != rows - 1, start + columns.get()),
            ] {
                if valid {
                    let probe = 1 << next;

                    if state & probe == 0 {
                        Self::helper(state | probe, rows, columns, next, end, result);
                    }
                }
            }
        }
    }

    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len() as _;
        let columns = NonZeroU8::new(grid.first().map_or(0, Vec::len) as u8).unwrap();
        let mut state = 0;
        let mut start = 0;
        let mut end = 0;
        let mut i = 0_u8;

        for row in &grid {
            for &value in row {
                match value {
                    -1 => state |= 1 << i,
                    0 => {}
                    1 => {
                        state |= 1 << i;
                        start = i;
                    }
                    _ => end = i,
                }

                i += 1;
            }
        }

        let mut result = 0;

        Self::helper(state, rows, columns, start, end, &mut result);

        result
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
