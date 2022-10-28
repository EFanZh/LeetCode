pub mod sliding_window;

pub trait Solution {
    fn number_of_substrings(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abcabc", 10), ("aaacb", 3), ("abc", 1)];

        for (s, expected) in test_cases {
            assert_eq!(S::number_of_substrings(s.to_string()), expected);
        }
    }
}
