pub mod iterative;

pub trait Solution {
    fn count_vowels(word: String) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aba", 6), ("abc", 3), ("ltcd", 0)];

        for (word, expected) in test_cases {
            assert_eq!(S::count_vowels(word.to_string()), expected);
        }
    }
}
