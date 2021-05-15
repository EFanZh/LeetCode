pub mod dynamic_programming;

pub trait Solution {
    fn min_distance(word1: String, word2: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("sea", "eat"), 2), (("leetcode", "etco"), 4), (("a", "a"), 0)];

        for ((word1, word2), expected) in test_cases.iter().copied() {
            assert_eq!(S::min_distance(word1.to_string(), word2.to_string()), expected);
        }
    }
}
