pub mod dynamic_programming;

pub trait Solution {
    fn is_match(s: String, p: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [
            (("aa", "a"), false),
            (("aa", "a*"), true),
            (("ab", ".*"), true),
            (("aab", "c*a*b"), true),
            (("mississippi", "c*a*b"), false),
        ];

        for ((s, p), expected) in test_cases.iter().copied() {
            assert_eq!(S::is_match(s.to_string(), p.to_string()), expected);
        }
    }
}
