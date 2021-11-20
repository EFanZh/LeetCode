pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let target = target as u32;

        let mut cars = position
            .into_iter()
            .zip(speed)
            .map(|(p, s)| (p as u32, s as u32))
            .collect::<Vec<_>>();

        cars.sort_unstable_by_key(|&(p, _)| Reverse(p));

        let mut result = 0;
        let mut prev_reach_time = (0, 1);

        for (p, s) in cars {
            let reach_time = (target - p, s);

            if u64::from(reach_time.0) * u64::from(prev_reach_time.1)
                > u64::from(prev_reach_time.0) * u64::from(reach_time.1)
            {
                result += 1;
                prev_reach_time = reach_time;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        Self::car_fleet(target, position, speed)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
