pub mod iterative;

pub trait Solution {
    fn merge_alternately(word1: String, word2: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abc", "pqr"), "apbqcr"),
            (("ab", "pqrs"), "apbqrs"),
            (("abcd", "pq"), "apbqcd"),
        ];

        for ((word1, word2), expected) in test_cases {
            assert_eq!(S::merge_alternately(word1.to_string(), word2.to_string()), expected);
        }
    }
}
