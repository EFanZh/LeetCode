pub mod mathematical;

pub trait Solution {
    fn count_days_together(arrive_alice: String, leave_alice: String, arrive_bob: String, leave_bob: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("08-15", "08-18", "08-16", "08-19"), 3),
            (("10-01", "10-31", "11-01", "12-31"), 0),
        ];

        for ((arrive_alice, leave_alice, arrive_bob, leave_bob), expected) in test_cases {
            assert_eq!(
                S::count_days_together(
                    arrive_alice.to_string(),
                    leave_alice.to_string(),
                    arrive_bob.to_string(),
                    leave_bob.to_string()
                ),
                expected,
            );
        }
    }
}
