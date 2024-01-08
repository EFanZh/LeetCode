pub mod greedy;

pub trait Solution {
    fn largest_merge(word1: String, word2: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("cabaa", "bcaaa"), "cbcabaaaaa"),
            (("abcabc", "abdcaba"), "abdcabcabcaba"),
        ];

        for ((word1, word2), expected) in test_cases {
            assert_eq!(S::largest_merge(word1.to_string(), word2.to_string()), expected);
        }
    }
}
