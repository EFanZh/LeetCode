pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::{NonZeroU32, NonZeroU64};

impl Solution {
    fn min_max(nums: &[NonZeroU32]) -> (NonZeroU32, NonZeroU32) {
        let mut iter = nums.iter().copied();
        let first = iter.next().unwrap();

        let (mut min, mut max) = if iter.len() % 2 == 0 {
            (first, first)
        } else {
            let second = iter.next().unwrap();

            if second < first {
                (second, first)
            } else {
                (first, second)
            }
        };

        let mut iter_2 = iter.clone();

        iter_2.next();

        for (x, y) in iter.zip(iter_2).step_by(2) {
            let (x, y) = if y < x { (y, x) } else { (x, y) };

            min = min.min(x);
            max = max.max(y);
        }

        (min, max)
    }

    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let time = time
            .into_iter()
            .map(|time| NonZeroU32::new(time as _).unwrap())
            .collect::<Vec<_>>();

        let total_trips = total_trips as u32;
        let (min, max) = Self::min_max(&time);
        let count = u64::from(total_trips.div_ceil(time.len() as u32));
        let mut left = u64::from(min.get()) * count;
        let mut right = u64::from(max.get()) * count;

        while left < right {
            let middle = (left + right) / 2;

            let trips = time.iter().try_fold(0_u32, |sum, &time| {
                (u64::from(sum) + middle / NonZeroU64::from(time)).try_into().ok()
            });
            if trips.is_none_or(|trips| trips >= total_trips) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        left as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        Self::minimum_time(time, total_trips)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
