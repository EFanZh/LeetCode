pub mod dynamic_programming;

pub trait Solution {
    fn longest_palindrome(word1: String, word2: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("cacb", "cbba"), 5), (("ab", "ab"), 3), (("aa", "bb"), 0)];

        for ((word1, word2), expected) in test_cases {
            assert_eq!(S::longest_palindrome(word1.to_string(), word2.to_string()), expected);
        }
    }
}
