pub mod iterative;

pub trait Solution {
    fn common_chars(words: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["bella", "label", "roller"] as &[_], &["e", "l", "l"] as &[_]),
            (&["cool", "lock", "cook"], &["c", "o"]),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::common_chars(words.iter().copied().map(str::to_string).collect())),
                expected
            );
        }
    }
}
