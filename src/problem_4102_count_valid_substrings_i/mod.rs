pub mod iterative;

pub trait Solution {
    fn count_valid_substrings(s: String) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abac", 10), ("abc", 5), ("ac", 3), ("aabc", 8)];

        for (s, expected) in test_cases {
            assert_eq!(S::count_valid_substrings(s.to_string()), expected);
        }
    }
}
