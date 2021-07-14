pub mod iterative;

pub trait Solution {
    fn valid_palindrome(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("aba", true),
            ("abca", true),
            ("abc", false),
            (
                "aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga",
                true,
            ),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::valid_palindrome(s.to_string()), expected);
        }
    }
}
