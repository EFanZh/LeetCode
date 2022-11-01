pub mod sliding_window;

pub trait Solution {
    fn balanced_string(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("QWER", 0),
            ("QQWE", 1),
            ("QQQW", 2),
            ("WQWRQQQW", 3),
            ("WWEQERQWQWWRWWERQWEQ", 4),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::balanced_string(s.to_string()), expected);
        }
    }
}
