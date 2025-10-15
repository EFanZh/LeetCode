pub mod bit_manipulation;

pub trait Solution {
    fn is_substring_present(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("leetcode", true), ("abcba", true), ("abcd", false)];

        for (s, expected) in test_cases {
            assert_eq!(S::is_substring_present(s.to_string()), expected);
        }
    }
}
