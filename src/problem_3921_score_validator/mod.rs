pub mod iterative;

pub trait Solution {
    fn score_validator(events: Vec<String>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["1", "4", "W", "6", "WD"] as &[_], &[12, 1] as &[_]),
            (&["WD", "NB", "0", "4", "4"], &[10, 0]),
            (&["W", "W", "W", "W", "W", "W", "W", "W", "W", "W", "W"], &[0, 10]),
        ];

        for (events, expected) in test_cases {
            assert_eq!(
                S::score_validator(events.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
