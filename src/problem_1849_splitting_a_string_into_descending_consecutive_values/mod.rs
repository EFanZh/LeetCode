pub mod backtracking;

pub trait Solution {
    fn split_string(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("1234", false),
            ("050043", true),
            ("9080701", false),
            ("99999999999999999998", true),
            ("00000", false),
            ("64424509442147483647", false),
            ("10009998", true),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::split_string(s.to_string()), expected);
        }
    }
}
