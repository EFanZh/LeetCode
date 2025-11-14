pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut iter = grid.iter_mut().map(Vec::as_mut_slice);
        let cache = iter.next().unwrap();

        let mut result = cache
            .iter()
            .fold((i32::MIN, i32::MAX / 2), |(result, min), &value| {
                (result.max(value - min), min.min(value))
            })
            .0;

        iter.for_each(|row| {
            let mut left = i32::MAX;

            cache.iter_mut().zip(&*row).for_each(|(target, &value)| {
                result = result.max(value - left.min(*target));
                *target = (*target).min(value);
                left = left.min(*target);
            });
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        Self::max_score(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
