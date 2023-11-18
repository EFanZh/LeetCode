pub mod brute_force;

pub trait Solution {
    fn num_of_strings(patterns: Vec<String>, word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["a", "abc", "bc", "d"] as &[_], "abc"), 3),
            ((&["a", "b", "c"], "aaaaabbbbb"), 2),
            ((&["a", "a", "a"], "ab"), 3),
        ];

        for ((patterns, word), expected) in test_cases {
            assert_eq!(
                S::num_of_strings(patterns.iter().copied().map(str::to_string).collect(), word.to_string()),
                expected,
            );
        }
    }
}
