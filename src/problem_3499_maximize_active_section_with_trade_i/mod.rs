pub mod greedy;

pub trait Solution {
    fn max_active_sections_after_trade(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("01", 1), ("0100", 4), ("1000100", 7)];

        for (s, expected) in test_cases {
            assert_eq!(S::max_active_sections_after_trade(s.to_string()), expected);
        }
    }
}
