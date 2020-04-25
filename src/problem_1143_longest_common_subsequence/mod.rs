pub mod dynamic_programming_1;
pub mod dynamic_programming_2;
pub mod memoized_dynamic_programming_1;
pub mod memoized_dynamic_programming_2;

pub trait Solution {
    fn longest_common_subsequence(text1: String, text2: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcde", "ace"), 3),
            (("abc", "abc"), 3),
            (("abc", "def"), 0),
            (("psnw", "vozsh"), 1),
            (("ac", "bc"), 1),
        ];

        for ((text1, text2), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::longest_common_subsequence(text1.to_string(), text2.to_string()),
                expected
            );
        }
    }
}
