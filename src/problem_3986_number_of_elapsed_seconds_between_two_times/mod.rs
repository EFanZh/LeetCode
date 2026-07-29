pub mod mathematical;

pub trait Solution {
    fn seconds_between_times(start_time: String, end_time: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("01:00:00", "01:00:25"), 25), (("12:34:56", "13:00:00"), 1504)];

        for ((start_time, end_time), expected) in test_cases {
            assert_eq!(
                S::seconds_between_times(start_time.to_string(), end_time.to_string()),
                expected,
            );
        }
    }
}
