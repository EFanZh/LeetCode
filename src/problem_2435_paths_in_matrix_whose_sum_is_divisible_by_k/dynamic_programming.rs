pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroUsize;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let columns = grid.first().map_or(0, Vec::len);
        let k_usize = NonZeroUsize::new(k as u32 as usize).unwrap();
        let mut cache = vec![0_u32; k_usize.get() * (columns + 1)].into_boxed_slice();
        let (left, cache) = cache.split_at_mut(k_usize.get());

        cache[0] = 1;

        for row in grid {
            let mut left = &*left;

            cache
                .chunks_exact_mut(k_usize.get())
                .zip(row)
                .for_each(|(counts, value)| {
                    counts.iter_mut().zip(left).for_each(|(target, &left)| {
                        *target += left;
                        *target = (*target).checked_sub(1_000_000_007).unwrap_or(*target);
                    });

                    counts.rotate_right(value as u32 as usize % k_usize);
                    left = counts;
                });
        }

        cache[cache.len() - k_usize.get()] as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        Self::number_of_paths(grid, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
