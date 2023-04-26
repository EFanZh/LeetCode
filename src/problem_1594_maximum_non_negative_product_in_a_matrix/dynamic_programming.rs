pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::convert::TryFrom;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let mut iter = grid.into_iter();
        let mut product = 1;

        let mut cache = iter
            .next()
            .unwrap()
            .into_iter()
            .map(|value| {
                product *= i64::from(value);

                (product, product)
            })
            .collect::<Box<_>>();

        for row in iter {
            let mut iter = cache.iter_mut().zip(row);
            let (target, value) = iter.next().unwrap();
            let value = i64::from(value);

            let mut left = match value.cmp(&0) {
                Ordering::Less => (target.1 * value, target.0 * value),
                Ordering::Equal => (0, 0),
                Ordering::Greater => (target.0 * value, target.1 * value),
            };

            *target = left;

            for (target, value) in iter {
                let value = i64::from(value);

                left = match value.cmp(&0) {
                    Ordering::Less => (target.1.max(left.1) * value, target.0.min(left.0) * value),
                    Ordering::Equal => (0, 0),
                    Ordering::Greater => (target.0.min(left.0) * value, target.1.max(left.1) * value),
                };

                *target = left;
            }
        }

        u64::try_from(cache.last().unwrap().1).map_or(-1, |value| (value % 1_000_000_007) as _)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        Self::max_product_path(grid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
