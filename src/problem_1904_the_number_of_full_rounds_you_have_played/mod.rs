pub mod mathematical;

pub trait Solution {
    fn number_of_rounds(login_time: String, logout_time: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("09:31", "10:14"), 1),
            (("21:30", "03:00"), 22),
            (("00:47", "00:57"), 0),
        ];

        for ((login_time, logout_time), expected) in test_cases {
            assert_eq!(
                S::number_of_rounds(login_time.to_string(), logout_time.to_string()),
                expected,
            );
        }
    }
}
