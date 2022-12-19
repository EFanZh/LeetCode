pub mod mathematical;

pub trait Solution {
    fn days_between_dates(date1: String, date2: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("2019-06-29", "2019-06-30"), 1),
            (("2020-01-15", "2019-12-31"), 15),
            (("2100-09-22", "1991-03-12"), 40006),
        ];

        for ((date1, date2), expected) in test_cases {
            assert_eq!(S::days_between_dates(date1.to_string(), date2.to_string()), expected);
        }
    }
}
