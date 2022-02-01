pub mod iterative;

pub trait Solution {
    fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["amazon", "apple", "facebook", "google", "leetcode"] as &[_],
                    &["e", "o"] as &[_],
                ),
                &["facebook", "google", "leetcode"] as &[_],
            ),
            (
                (&["amazon", "apple", "facebook", "google", "leetcode"], &["l", "e"]),
                &["apple", "google", "leetcode"],
            ),
        ];

        for ((words1, words2), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::word_subsets(
                    words1.iter().copied().map(str::to_string).collect(),
                    words2.iter().copied().map(str::to_string).collect()
                )),
                expected
            );
        }
    }
}
