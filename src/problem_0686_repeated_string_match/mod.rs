pub mod search_substring;

pub trait Solution {
    fn repeated_string_match(a: String, b: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcd", "cdabcdab"), 3),
            (("a", "aa"), 2),
            (("a", "a"), 1),
            (("abc", "wxyz"), -1),
        ];

        for ((a, b), expected) in test_cases {
            assert_eq!(S::repeated_string_match(a.to_string(), b.to_string()), expected);
        }
    }
}
