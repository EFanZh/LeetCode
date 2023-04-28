pub mod sliding_window;

pub trait Solution {
    fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["daniel", "daniel", "daniel", "luis", "luis", "luis", "luis"] as &[_],
                    &["10:00", "10:40", "11:00", "09:00", "11:00", "13:00", "15:00"] as &[_],
                ),
                &["daniel"] as &[_],
            ),
            (
                (
                    &["alice", "alice", "alice", "bob", "bob", "bob", "bob"],
                    &["12:01", "12:00", "18:00", "21:00", "21:20", "21:30", "23:00"],
                ),
                &["bob"],
            ),
        ];

        for ((key_name, key_time), expected) in test_cases {
            assert_eq!(
                S::alert_names(
                    key_name.iter().copied().map(str::to_string).collect(),
                    key_time.iter().copied().map(str::to_string).collect(),
                ),
                expected,
            );
        }
    }
}
