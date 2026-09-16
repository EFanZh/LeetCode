pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    pub fn min_penalty(period: i32, lights: Vec<i32>, arrival_time: Vec<i32>) -> i32 {
        let period = NonZero::new(period.cast_unsigned()).unwrap();
        let max_green = lights.into_iter().map(i32::cast_unsigned).fold(0, u32::max);

        arrival_time
            .into_iter()
            .fold(0, |penalty, arrival_time| {
                let arrival_time = arrival_time.cast_unsigned() % period;

                u32::max(
                    penalty,
                    if arrival_time < max_green {
                        0
                    } else {
                        period.get() - arrival_time
                    },
                )
            })
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_penalty(period: i32, lights: Vec<i32>, arrival_time: Vec<i32>) -> i32 {
        Self::min_penalty(period, lights, arrival_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
