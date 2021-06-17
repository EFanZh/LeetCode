pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn longest_palindrome(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abccccdd", 7), ("a", 1), ("bb", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::longest_palindrome(s.to_string()), expected);
        }
    }
}
