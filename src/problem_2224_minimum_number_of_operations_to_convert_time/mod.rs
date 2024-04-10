pub mod iterative;

pub trait Solution {
    fn convert_time(current: String, correct: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("02:30", "04:35"), 3), (("11:00", "11:01"), 1)];

        for ((current, correct), expected) in test_cases {
            assert_eq!(S::convert_time(current.to_string(), correct.to_string()), expected);
        }
    }
}
