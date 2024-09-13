pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(grid: &[Vec<i32>], mut start: usize, step: usize, cache: &mut [u16], result: &mut u32) {
        while let Some(row) = grid.get(start) {
            start = start.wrapping_add(step);

            let mut prev = 0;

            for (target, &cell) in cache.iter_mut().zip(row) {
                *target = if cell == 0 { 0 } else { prev.min(*target) + 1 };
                prev = *target;
            }

            prev = 0;

            for target in cache.iter_mut().rev() {
                *target = (*target).min(prev + 1);
                prev = *target;
                *result += u32::from(target.saturating_sub(1));
            }
        }
    }

    pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut cache = vec![0; grid.first().map_or(0, Vec::len)].into_boxed_slice();

        Self::helper(&grid, 0, 1, &mut cache, &mut result);
        cache.fill(0);
        Self::helper(&grid, grid.len().wrapping_sub(1), usize::MAX, &mut cache, &mut result);

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        Self::count_pyramids(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
