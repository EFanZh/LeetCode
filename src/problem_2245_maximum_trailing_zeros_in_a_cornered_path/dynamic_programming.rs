pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse(value: i32) -> u8 {
        let mut value = value as u32;
        let mut twos = 0;
        let mut fives = 0;

        while value % 2 == 0 {
            value /= 2;
            twos += 1;
        }

        while value % 5 == 0 {
            value /= 5;
            fives += 1;
        }

        (fives << 4) | twos
    }

    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let columns = grid.first().map_or(0, Vec::len);
        let cache_size = columns * rows;
        let mut factors = vec![0_u8; cache_size].into_boxed_slice();
        let mut cache = vec![(0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32); cache_size].into_boxed_slice();

        factors
            .chunks_exact_mut(columns)
            .zip(grid)
            .for_each(|(target_row, source_row)| {
                target_row
                    .iter_mut()
                    .zip(source_row)
                    .for_each(|(target, source)| *target = Self::parse(source));
            });

        // Top down.

        let mut iter = cache.chunks_exact_mut(columns).zip(factors.chunks_exact(columns));
        let (mut top_cache_row, first_factor_row) = iter.next().unwrap();
        let mut left = (0, 0);

        top_cache_row
            .iter_mut()
            .zip(first_factor_row)
            .for_each(|(target, factors)| {
                let two_factors = u32::from(factors & ((1 << 4) - 1));
                let five_factors = u32::from(factors >> 4);

                left.0 += two_factors;
                left.1 += five_factors;
                (target.0, target.1) = (two_factors, five_factors);
                (target.2, target.3) = left;
            });

        iter.for_each(|(cache_row, factor_row)| {
            let mut left = (0, 0);

            cache_row
                .iter_mut()
                .zip(&*top_cache_row)
                .zip(factor_row)
                .for_each(|((target, top), &factors)| {
                    let two_factors = u32::from(factors & ((1 << 4) - 1));
                    let five_factors = u32::from(factors >> 4);

                    left.0 += two_factors;
                    left.1 += five_factors;
                    (target.0, target.1) = (top.0 + two_factors, top.1 + five_factors);
                    (target.2, target.3) = left;
                });

            top_cache_row = cache_row;
        });

        // Bottom up.

        let mut iter = cache.chunks_exact_mut(columns).zip(factors.chunks_exact(columns));
        let (mut bottom_cache_row, last_factor_row) = iter.next_back().unwrap();
        let mut right = (0, 0);

        bottom_cache_row
            .iter_mut()
            .zip(last_factor_row)
            .rev()
            .for_each(|(target, factors)| {
                let two_factors = u32::from(factors & ((1 << 4) - 1));
                let five_factors = u32::from(factors >> 4);

                right.0 += two_factors;
                right.1 += five_factors;
                (target.4, target.5) = right;
                (target.6, target.7) = (two_factors, five_factors);
            });

        iter.rev().for_each(|(cache_row, factor_row)| {
            let mut right = (0, 0);

            cache_row
                .iter_mut()
                .zip(&*bottom_cache_row)
                .zip(factor_row)
                .rev()
                .for_each(|((target, bottom), &factors)| {
                    let two_factors = u32::from(factors & ((1 << 4) - 1));
                    let five_factors = u32::from(factors >> 4);

                    right.0 += two_factors;
                    right.1 += five_factors;
                    (target.4, target.5) = right;
                    (target.6, target.7) = (bottom.6 + two_factors, bottom.7 + five_factors);
                });

            bottom_cache_row = cache_row;
        });

        // Calculate result.

        cache.iter().zip(&*factors).fold(0_u32, |result, (values, &factors)| {
            let two_factors = u32::from(factors & ((1 << 4) - 1));
            let five_factors = u32::from(factors >> 4);

            result
                .max((values.0 + values.2 - two_factors).min(values.1 + values.3 - five_factors))
                .max((values.0 + values.4 - two_factors).min(values.1 + values.5 - five_factors))
                .max((values.2 + values.6 - two_factors).min(values.3 + values.7 - five_factors))
                .max((values.4 + values.6 - two_factors).min(values.5 + values.7 - five_factors))
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
