pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn is_valid(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("aabcbc", true),
            ("abcabcababcc", true),
            ("abccba", false),
            ("aabcabcbc", true),
            ("aabbcc", false),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::is_valid(s.to_string()), expected);
        }
    }
}
