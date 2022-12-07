pub mod sliding;

pub trait Solution {
    fn distinct_echo_substrings(text: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abcabcabc", 3), ("leetcodeleetcode", 2)];

        for (text, expected) in test_cases {
            assert_eq!(S::distinct_echo_substrings(text.to_string()), expected);
        }
    }
}
