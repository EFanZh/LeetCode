pub mod iterative;

pub trait Solution {
    fn check_almost_equivalent(word1: String, word2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("aaaa", "bccb"), false),
            (("abcdeef", "abaaacc"), true),
            (("cccddabba", "babababab"), true),
        ];

        for ((word1, word2), expected) in test_cases {
            assert_eq!(
                S::check_almost_equivalent(word1.to_string(), word2.to_string()),
                expected
            );
        }
    }
}
