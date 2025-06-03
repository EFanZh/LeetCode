pub mod greedy;

pub trait Solution {
    fn make_smallest_palindrome(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("egcfe", "efcfe"), ("abcd", "abba"), ("seven", "neven")];

        for (s, expected) in test_cases {
            assert_eq!(S::make_smallest_palindrome(s.to_string()), expected);
        }
    }
}
