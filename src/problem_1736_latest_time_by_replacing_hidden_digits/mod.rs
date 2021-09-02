pub mod brute_force;

pub trait Solution {
    fn maximum_time(time: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("2?:?0", "23:50"),
            ("0?:3?", "09:39"),
            ("1?:22", "19:22"),
            ("?0:12", "20:12"),
            ("?4:32", "14:32"),
            ("??:01", "23:01"),
            ("00:01", "00:01")
        ];

        for (time, expected) in test_cases {
            assert_eq!(S::maximum_time(time.to_string()), expected);
        }
    }
}
