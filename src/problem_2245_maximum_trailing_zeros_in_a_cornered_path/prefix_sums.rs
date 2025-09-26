pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse(value: i32) -> u64 {
        let mut value = value as u32;
        let mut twos = 0;
        let mut fives = 0;

        while value.is_multiple_of(2) {
            value /= 2;
            twos += 1;
        }

        while value.is_multiple_of(5) {
            value /= 5;
            fives += 1;
        }

        (fives << 32) | twos
    }

    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let cache_size = columns * rows;
        let mut cache = vec![0_u64; cache_size].into_boxed_slice();
        let mut iter = cache.chunks_exact_mut(columns).zip(grid);
        let (mut prev_cache_row, first_row) = iter.next().unwrap();
        let mut left = 0;

        prev_cache_row.iter_mut().zip(first_row).for_each(|(target, value)| {
            left += Self::parse(value);
            *target = left;
        });

        iter.for_each(|(cache_row, row)| {
            let mut left = 0;

            cache_row
                .iter_mut()
                .zip(&*prev_cache_row)
                .zip(row)
                .for_each(|((target, top), value)| {
                    left += Self::parse(value);
                    *target = left + top;
                });

            prev_cache_row = cache_row;
        });

        let get_cache = |y, x| {
            (x < columns)
                .then(|| cache.get(columns.wrapping_mul(y).wrapping_add(x)).copied())
                .flatten()
                .unwrap_or(0)
        };

        let extract = |x| u32::min((x >> 32) as _, x as _);

        cache
            .chunks_exact(columns)
            .enumerate()
            .fold(0, |result, (y, cache_row)| {
                let top_row_end = get_cache(y.wrapping_sub(1), columns - 1);
                let row_end = get_cache(y, columns - 1);

                cache_row.iter().enumerate().fold(result, |result, (x, &counts)| {
                    let top_left = get_cache(y.wrapping_sub(1), x.wrapping_sub(1));
                    let top = get_cache(y.wrapping_sub(1), x);
                    let left = get_cache(y, x.wrapping_sub(1));
                    let left_column_end = get_cache(rows - 1, x.wrapping_sub(1));
                    let column_end = get_cache(rows - 1, x);

                    let candidate_1 = counts - top_left;
                    let candidate_2 = (row_end - left) - (top_row_end - top);
                    let candidate_3 = (column_end - top) - (left_column_end - left);
                    let candidate_4 = (row_end + column_end - counts) - (top_row_end + left_column_end - top_left);

                    result.max(u32::max(
                        u32::max(extract(candidate_1), extract(candidate_2)),
                        u32::max(extract(candidate_3), extract(candidate_4)),
                    ))
                })
            }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        Self::max_trailing_zeros(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
