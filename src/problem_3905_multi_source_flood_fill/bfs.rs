pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

impl Solution {
    pub fn color_grid(n: i32, m: i32, sources: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const VISITING: i32 = i32::MIN;
        const MASK: i32 = i32::MAX;

        let n = n.cast_unsigned() as usize;
        let m = m.cast_unsigned() as usize;
        let mut result = vec![vec![0_i32; m]; n];

        let mut queue = sources
            .into_iter()
            .map(|source| {
                let [row, column, color] = <[_; 3]>::map(source.try_into().ok().unwrap(), i32::cast_unsigned);

                result[row as usize][column as usize] = color.cast_signed();

                (row, column)
            })
            .collect::<VecDeque<_>>();

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let (row, column) = queue.pop_front().unwrap();
                let row = row as usize;
                let column = column as usize;
                let color = result[row][column];

                for (next_row, next_column) in [
                    (row.wrapping_sub(1), column),
                    (row, column.wrapping_sub(1)),
                    (row, column + 1),
                    (row + 1, column),
                ] {
                    if let Some(state) = result.get_mut(next_row).and_then(|row| row.get_mut(next_column)) {
                        *state = if *state == 0 {
                            queue.push_back((next_row as _, next_column as _));

                            color | VISITING
                        } else if *state & VISITING == 0 {
                            continue;
                        } else {
                            u32::max((*state & MASK).cast_unsigned(), color.cast_unsigned()).cast_signed() | VISITING
                        };
                    }
                }
            }

            for &(row, column) in &queue {
                result[row as usize][column as usize] &= MASK;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn color_grid(n: i32, m: i32, sources: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::color_grid(n, m, sources)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
