pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn seconds_between_times(start_time: String, end_time: String) -> i32 {
        let [
            start_hour_10,
            start_hour_1,
            _,
            start_minute_10,
            start_minute_1,
            _,
            start_second_10,
            start_second_1,
        ] = start_time.into_bytes().try_into().ok().unwrap();

        let [
            end_hour_10,
            end_hour_1,
            _,
            end_minute_10,
            end_minute_1,
            _,
            end_second_10,
            end_second_1,
        ] = end_time.into_bytes().try_into().ok().unwrap();

        (i32::from(end_hour_10.wrapping_sub(start_hour_10).cast_signed())) * 36000
            + (i32::from(end_hour_1.wrapping_sub(start_hour_1).cast_signed())) * 3600
            + (i32::from(end_minute_10.wrapping_sub(start_minute_10).cast_signed())) * 600
            + (i32::from(end_minute_1.wrapping_sub(start_minute_1).cast_signed())) * 60
            + (i32::from(end_second_10.wrapping_sub(start_second_10).cast_signed())) * 10
            + (i32::from(end_second_1.wrapping_sub(start_second_1).cast_signed()))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn seconds_between_times(start_time: String, end_time: String) -> i32 {
        Self::seconds_between_times(start_time, end_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
