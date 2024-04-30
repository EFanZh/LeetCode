pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::{NonZeroU32, NonZeroU64};

impl Solution {
    fn check(dist: &[i32], last_dist: i32, hour: u64, speed: NonZeroU32) -> bool {
        let speed = NonZeroU64::from(speed);
        let mut cost = 0;

        for &dist in dist {
            cost += (u64::from(dist as u32) + (speed.get() - 1)) / speed;
        }

        (cost * speed.get() + u64::from(last_dist as u32)) * 100 > hour * speed.get()
    }

    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let (&last_dist, dist) = dist.split_last().unwrap();
        let hour = (hour * 100.0).round() as u64;
        let mut start = 1;
        let mut count = u32::MAX - 1;

        loop {
            let half = count / 2;
            let middle = start + half;

            if Self::check(dist, last_dist, hour, NonZeroU32::new(middle).unwrap()) {
                start = middle + 1;
                count -= half + 1;
            } else {
                count = half;
            }

            if count == 0 {
                break;
            }
        }

        start as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        Self::min_speed_on_time(dist, hour)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
