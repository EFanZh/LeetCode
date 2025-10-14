pub mod greedy;

pub trait Solution {
    fn minimize_string_value(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("???", "abc"),
            ("a?a?", "abac"),
            ("abcdefghijklmnopqrstuvwxy??", "abcdefghijklmnopqrstuvwxyaz"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::minimize_string_value(s.to_string()), expected);
        }
    }
}
