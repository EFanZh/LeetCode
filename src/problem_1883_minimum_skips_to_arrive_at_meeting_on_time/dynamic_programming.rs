pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;
use std::num::NonZeroU32;

impl Solution {
    fn make_whole(mut x: u32, base: NonZeroU32) -> u32 {
        x += base.get() - 1;

        x - x % base
    }

    pub fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        let dist = dist.into_iter().map(|x| x as u32).collect::<Vec<_>>();
        let speed = NonZeroU32::new(speed as _).unwrap();
        let hours_before = hours_before as u32;

        speed.get().checked_mul(hours_before).map_or(0, |scaled_hours_before| {
            let n = dist.len();
            let mut cache = vec![0; n].into_boxed_slice();

            for &road in &dist {
                let mut top_left = u32::MAX;

                for target in &mut *cache {
                    top_left = mem::replace(target, Self::make_whole(*target, speed).min(top_left) + road);
                }
            }

            let i = cache.partition_point(|&x| x > scaled_hours_before);

            if i == n { -1 } else { i as _ }
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
        Self::min_skips(dist, speed, hours_before)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
