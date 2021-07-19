pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let n = n as usize;
        let row = row as usize;
        let column = column as usize;
        let half = (n + 1) / 2;
        let last_index = n - 1;
        let cache_size = (1 + half) * half / 2;
        let mut cache_and_temp = vec![1.0; cache_size * 2].into_boxed_slice();
        let (mut cache, mut temp) = cache_and_temp.split_at_mut(cache_size);
        let get_index_unchecked = |row, column| (1 + row) * row / 2 + column;

        let get_index = |row, column| {
            let row = if row < half { row } else { last_index - row };
            let column = if column < half { column } else { last_index - column };
            let (row, column) = if row < column { (column, row) } else { (row, column) };

            get_index_unchecked(row, column)
        };

        for _ in 0..k {
            for row in 0..half {
                for column in 0..=row {
                    let mut sum = 0.0;

                    for &(row_offset, column_offset) in
                        &[(-2, -1), (-2, 1), (-1, -2), (-1, 2), (1, -2), (1, 2), (2, -1), (2, 1)]
                    {
                        let next_row = row.wrapping_add(row_offset as _);
                        let next_column = column.wrapping_add(column_offset as _);

                        if next_row < n && next_column < n {
                            sum += cache[get_index(next_row, next_column)];
                        }
                    }

                    temp[get_index_unchecked(row, column)] = sum / 8.0;
                }
            }

            mem::swap(&mut cache, &mut temp);
        }

        cache[get_index(row, column)]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        Self::knight_probability(n, k, row, column)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
