pub mod b_tree_map;
pub mod hash_map;
pub mod trie;

pub trait Solution {
    fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &["abcd", "dcba", "lls", "s", "sssll"] as &[_],
                &[[0, 1], [1, 0], [2, 4], [3, 2]] as &[_],
            ),
            (&["bat", "tab", "cat"], &[[0, 1], [1, 0]]),
            (&["a", ""], &[[0, 1], [1, 0]]),
        ];

        for (words, expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(S::palindrome_pairs(
                    words.iter().copied().map(str::to_string).collect()
                )),
                expected
            );
        }
    }
}
