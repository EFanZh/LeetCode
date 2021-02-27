pub mod match_by_chunks;

pub trait Solution {
    fn is_match(s: String, p: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("aa", "a"), false),
            (("aa", "*"), true),
            (("cb", "?a"), false),
            (("adceb", "*a*b"), true),
            (("acdcb", "a*c?b"), false),
            (("aaaa", "***a"), true),
            (("mississippi", "m??*ss*?i*pi"), false),
            (("b", "?*?"), false),
            (("a", "aa"), false),
        ];

        for ((s, p), expected) in test_cases.iter().copied() {
            assert_eq!(S::is_match(s.to_string(), p.to_string()), expected);
        }
    }
}
