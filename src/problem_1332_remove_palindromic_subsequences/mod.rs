pub mod iterative;

pub trait Solution {
    fn remove_palindrome_sub(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("ababa", 1), ("abb", 2), ("baabb", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::remove_palindrome_sub(s.to_string()), expected);
        }
    }
}
