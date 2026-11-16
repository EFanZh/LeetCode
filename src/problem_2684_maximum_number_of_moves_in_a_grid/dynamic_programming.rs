pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let grid = grid
            .into_iter()
            .map(|row| row.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let mut cache = vec![0_i16; rows].into_boxed_slice();
        let mut result = 0;

        for column in 1..columns {
            let mut max_length = 0;
            let mut prev = 0;

            for (i, row) in grid.iter().enumerate() {
                let value = row[column];

                let mut max_moves = if grid
                    .get(i.wrapping_sub(1))
                    .is_some_and(|top_row| top_row[column - 1] < value)
                {
                    prev + 1
                } else {
                    i16::MIN
                };

                if row[column - 1] < value {
                    max_moves = max_moves.max(cache[i] + 1);
                }

                if grid.get(i + 1).is_some_and(|bottom_row| bottom_row[column - 1] < value) {
                    max_moves = max_moves.max(cache[i + 1] + 1);
                }

                let target = &mut cache[i];

                prev = mem::replace(target, max_moves);
                max_length = max_length.max(*target);
            }

            if max_length <= 0 {
                break;
            }

            result = result.max(max_length.cast_unsigned());
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        Self::max_moves(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
