pub mod greedy;

pub trait Solution {
    fn break_palindrome(palindrome: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abccba", "aaccba"), ("a", ""), ("aa", "ab"), ("aabaa", "aabab")];

        for (palindrome, expected) in test_cases {
            assert_eq!(S::break_palindrome(palindrome.to_string()), expected);
        }
    }
}
