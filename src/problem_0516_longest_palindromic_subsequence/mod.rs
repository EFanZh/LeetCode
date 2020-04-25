pub mod dynamic_programming;
pub mod memoized_dynamic_programming;

pub trait Solution {
    fn longest_palindrome_subseq(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("", 0), ("a", 1), ("bbbab", 4), ("cbbd", 2), ("character", 5)];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::longest_palindrome_subseq(s.to_string()), expected);
        }
    }
}
