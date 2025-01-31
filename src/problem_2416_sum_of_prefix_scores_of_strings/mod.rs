pub mod trie;

pub trait Solution {
    fn sum_prefix_scores(words: Vec<String>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["abc", "ab", "bc", "b"] as &[_], &[5, 4, 3, 2] as &[_]),
            (&["abcd"], &[4]),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::sum_prefix_scores(words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
