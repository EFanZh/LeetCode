pub mod greedy;

pub trait Solution {
    fn max_palindromes_after_operations(words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["abbb", "ba", "aa"] as &[_], 3),
            (&["abc", "ab"], 2),
            (&["cd", "ef", "a"], 1),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::max_palindromes_after_operations(words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
