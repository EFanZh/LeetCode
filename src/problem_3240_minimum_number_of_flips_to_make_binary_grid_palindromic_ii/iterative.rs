pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn index_fn(index: usize) -> impl for<'a> Fn(&'a Vec<u32>) -> Option<&u32> + Copy {
        move |v| v.get(index)
    }

    fn iter_both_end<T>(s: &[T]) -> impl DoubleEndedIterator<Item = (&T, &T)> {
        let (left, right) = s.split_at(s.len() / 2);

        left.iter().zip(right.iter().rev())
    }

    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let grid = grid
            .into_iter()
            .map(|row| row.into_iter().map(i32::cast_unsigned).collect())
            .collect::<Vec<Vec<_>>>();

        let mut result = 0;

        Self::iter_both_end(&grid).for_each(|(top_row, bottom_row)| {
            Self::iter_both_end(top_row)
                .zip(Self::iter_both_end(bottom_row))
                .for_each(|((&x, &y), (&z, &w))| {
                    let ones = x + y + z + w;

                    result += ones.min(4 - ones);
                });
        });

        let mut total_ones = 0;
        let mut diff_pairs = 0;

        if !grid.len().is_multiple_of(2) {
            let middle_row = &grid[grid.len() / 2];
            let half_row_len = middle_row.len() / 2;

            if !middle_row.len().is_multiple_of(2) {
                result += middle_row[half_row_len];
            }

            Self::iter_both_end(middle_row).for_each(|(&x, &y)| {
                total_ones += x + y;
                diff_pairs += x ^ y;
            });
        }

        let columns = grid.first().map_or(0, Vec::len);

        if !columns.is_multiple_of(2) {
            let (top, bottom) = grid.split_at(grid.len() / 2);
            let get_value = Self::index_fn(columns / 2);

            top.iter()
                .filter_map(get_value)
                .zip(bottom.iter().filter_map(get_value).rev())
                .for_each(|(&x, &y)| {
                    total_ones += x + y;
                    diff_pairs += x ^ y;
                });
        }

        result += diff_pairs;

        if ((total_ones - diff_pairs) & 2 != 0) && diff_pairs == 0 {
            result += 2;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        Self::min_flips(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
