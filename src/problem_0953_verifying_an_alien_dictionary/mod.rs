pub mod iterative;

pub trait Solution {
    fn is_alien_sorted(words: Vec<String>, order: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["hello", "leetcode"] as &[_], "hlabcdefgijkmnopqrstuvwxyz"), true),
            ((&["word", "world", "row"], "worldabcefghijkmnpqstuvxyz"), false),
            ((&["apple", "app"], "abcdefghijklmnopqrstuvwxyz"), false),
        ];

        for ((words, order), expected) in test_cases {
            assert_eq!(
                S::is_alien_sorted(words.iter().copied().map(str::to_string).collect(), order.to_string()),
                expected
            );
        }
    }
}
