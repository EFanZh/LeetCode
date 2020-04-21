pub mod cheating;
pub mod naive;

pub trait Solution {
    fn str_str(haystack: String, needle: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [(("hello", "ll"), 2), (("aaaaa", "bba"), -1)];

        for ((haystack, needle), expected) in test_cases.iter().copied() {
            assert_eq!(S::str_str(haystack.to_string(), needle.to_string()), expected);
        }
    }
}
