pub mod bfs;

pub trait Solution {
    fn k_similarity(s1: String, s2: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("ab", "ba"), 1),
            (("abc", "bca"), 2),
            (("abac", "baca"), 2),
            (("aabbccd", "aabbccd"), 0),
            (("aabc", "abca"), 2),
            (("abcbca", "ababcc"), 1),
            (("bccaba", "abacbc"), 3),
            (("abbcac", "abcbca"), 2),
            (("aabbccddee", "cdacbeebad"), 6),
        ];

        for ((s1, s2), expected) in test_cases {
            assert_eq!(S::k_similarity(s1.to_string(), s2.to_string()), expected);
        }
    }
}
