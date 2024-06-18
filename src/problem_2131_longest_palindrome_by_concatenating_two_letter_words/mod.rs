pub mod greedy;

pub trait Solution {
    fn longest_palindrome(words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["lc", "cl", "gg"] as &[_], 6),
            (&["ab", "ty", "yt", "lc", "cl", "ab"], 8),
            (&["cc", "ll", "xx"], 2),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::longest_palindrome(words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
