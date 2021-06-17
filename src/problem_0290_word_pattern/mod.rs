pub mod hash_map;
pub mod index_hash_map;

pub trait Solution {
    fn word_pattern(pattern: String, s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abba", "dog cat cat dog"), true),
            (("abba", "dog cat cat fish"), false),
            (("aaaa", "dog cat cat dog"), false),
            (("abba", "dog dog dog dog"), false),
            (("aaa", "aa aa aa aa"), false),
        ];

        for ((pattern, s), expected) in test_cases {
            assert_eq!(S::word_pattern(pattern.to_string(), s.to_string()), expected);
        }
    }
}
