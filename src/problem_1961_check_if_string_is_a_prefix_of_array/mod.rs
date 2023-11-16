pub mod dynamic_programming;

pub trait Solution {
    fn is_prefix_string(s: String, words: Vec<String>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("iloveleetcode", &["i", "love", "leetcode", "apples"] as &[_]), true),
            (("iloveleetcode", &["apples", "i", "love", "leetcode"]), false),
            (("z", &["z"]), true),
            (("ccccccccc", &["c", "cc"]), false),
        ];

        for ((s, words), expected) in test_cases {
            assert_eq!(
                S::is_prefix_string(s.to_string(), words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
