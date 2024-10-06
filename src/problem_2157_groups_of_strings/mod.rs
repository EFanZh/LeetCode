pub mod hash_map;

pub trait Solution {
    fn group_strings(words: Vec<String>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["a", "b", "ab", "cde"] as &[_], [2, 3]),
            (&["a", "ab", "abc"], [1, 3]),
            (&["web", "a", "te", "hsx", "v", "k", "a", "roh"], [5, 4]),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::group_strings(words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
