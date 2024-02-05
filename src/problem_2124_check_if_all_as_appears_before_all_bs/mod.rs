pub mod iterative;

pub trait Solution {
    fn check_string(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("aaabbb", true),
            ("abab", false),
            ("bbb", true),
            ("a", true),
            ("aa", true),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::check_string(s.to_string()), expected);
        }
    }
}
