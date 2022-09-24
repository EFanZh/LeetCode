pub mod greedy;

pub trait Solution {
    fn break_palindrome(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abccba", "aaccba"), ("a", ""), ("aa", "ab"), ("aabaa", "aabab")];

        for (s, expected) in test_cases {
            assert_eq!(S::break_palindrome(s.to_string()), expected);
        }
    }
}
