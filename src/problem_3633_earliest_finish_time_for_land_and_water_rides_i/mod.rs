pub mod iterative;

pub trait Solution {
    fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 8] as &[_], &[4, 1] as &[_], &[6] as &[_], &[3] as &[_]), 9),
            ((&[5], &[3], &[1], &[10]), 14),
        ];

        for ((land_start_time, land_duration, water_start_time, water_duration), expected) in test_cases {
            assert_eq!(
                S::earliest_finish_time(
                    land_start_time.to_vec(),
                    land_duration.to_vec(),
                    water_start_time.to_vec(),
                    water_duration.to_vec(),
                ),
                expected,
            );
        }
    }
}
