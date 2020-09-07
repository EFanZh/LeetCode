pub mod brute_force;
pub mod kmp;

pub trait Solution {
    fn shortest_palindrome(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aacecaaa", "aaacecaaa"), ("abcd", "dcbabcd")];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::shortest_palindrome(s.to_string()), expected);
        }
    }
}
