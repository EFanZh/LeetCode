pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut iter = grid.iter_mut().map(Vec::as_mut_slice);
        let cache = iter.next().unwrap();
        let mut cache_iter = cache.iter_mut();

        while let Some(x) = cache_iter.next() {
            if *x == 0 {
                *x = 1;
            } else {
                cache_iter.for_each(|x| *x = 0);

                break;
            }
        }

        for row in iter {
            let mut right_count = 0;

            cache.iter_mut().zip(row).for_each(|(down_count, cell)| {
                if *cell == 0 {
                    right_count += *down_count;

                    let right_count_unsigned = right_count.cast_unsigned();

                    right_count = right_count_unsigned
                        .checked_sub(1_000_000_007)
                        .unwrap_or(right_count_unsigned)
                        .cast_signed();

                    *down_count = right_count;
                } else {
                    mem::swap(down_count, &mut right_count);
                }
            });
        }

        *cache.last().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        Self::unique_paths(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
