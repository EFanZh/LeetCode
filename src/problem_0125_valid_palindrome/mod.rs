pub mod bidirectional_search;

pub trait Solution {
    fn is_palindrome(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("A man, a plan, a canal: Panama", true),
            ("race a car", false),
            ("0P", false),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::is_palindrome(s.to_string()), expected);
        }
    }
}
