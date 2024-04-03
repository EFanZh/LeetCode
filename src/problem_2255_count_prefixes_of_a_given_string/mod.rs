pub mod iterative;

pub trait Solution {
    fn count_prefixes(words: Vec<String>, s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["a", "b", "c", "ab", "bc", "abc"] as &[_], "abc"), 3),
            ((&["a", "a"], "aa"), 2),
        ];

        for ((words, s), expected) in test_cases {
            assert_eq!(
                S::count_prefixes(words.iter().copied().map(str::to_string).collect(), s.to_string()),
                expected,
            );
        }
    }
}
