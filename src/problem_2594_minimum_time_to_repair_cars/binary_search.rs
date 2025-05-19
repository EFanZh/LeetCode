pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;
use std::num::{NonZeroU32, NonZeroU64};

impl Solution {
    fn min_max(values: &[NonZeroU32]) -> Option<(NonZeroU32, NonZeroU32)> {
        values.split_first().map(|(&first, rest)| {
            let mut min = first;
            let mut max = first;
            let mut iter = rest.chunks_exact(2);

            iter.by_ref().for_each(|chunk| {
                let [mut x, mut y] = chunk.try_into().unwrap();

                if y < x {
                    mem::swap(&mut x, &mut y);
                }

                if x < min {
                    min = x;
                }

                if y > max {
                    max = y;
                }
            });

            if let [last] = *iter.remainder() {
                if last < min {
                    min = last;
                } else if last > max {
                    max = last;
                }
            }

            (min, max)
        })
    }

    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let n = NonZeroU32::new(ranks.len() as _).unwrap();

        let ranks = ranks
            .into_iter()
            .filter_map(|rank| NonZeroU32::new(rank as _))
            .collect::<Vec<_>>();

        let cars = cars as u32;
        let (min_rank, max_rank) = Self::min_max(&ranks).unwrap();
        let factor = u64::from((cars / n) + u32::from(cars % n != 0)).pow(2);
        let mut left = factor * u64::from(min_rank.get());
        let mut right = factor * u64::from(max_rank.get());

        while left < right {
            let middle = u64::midpoint(left, right);
            let mut count = 0;

            for &rank in &ranks {
                count += (middle / NonZeroU64::from(rank)).isqrt() as u32;
            }

            if count < cars {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        Self::repair_cars(ranks, cars)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
