pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn normalize(v: Vec<i32>) -> Vec<u32> {
        v.into_iter().map(i32::cast_unsigned).collect()
    }

    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        let land_start_time = Self::normalize(land_start_time);
        let land_duration = Self::normalize(land_duration);
        let water_start_time = Self::normalize(water_start_time);
        let water_duration = Self::normalize(water_duration);

        let min_land_end_time = land_start_time
            .iter()
            .zip(&land_duration)
            .fold(u32::MAX, |min_land_end_time, (start, duration)| {
                min_land_end_time.min(start + duration)
            });

        let (min_water_end_time, candidate) = water_start_time.iter().zip(&water_duration).fold(
            (u32::MAX, u32::MAX),
            |(min_water_end_time, candidate), (start, &duration)| {
                (
                    min_water_end_time.min(start + duration),
                    candidate.min(min_land_end_time.max(*start) + duration),
                )
            },
        );

        land_start_time
            .iter()
            .zip(&land_duration)
            .fold(candidate, |min, (&start, duration)| {
                min.min(min_water_end_time.max(start) + duration)
            })
            .cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        Self::earliest_finish_time(land_start_time, land_duration, water_start_time, water_duration)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
