pub mod iterative;

pub trait Solution {
    fn possible_string_count(word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abbcccc", 5), ("abcd", 1), ("aaaa", 4)];

        for (word, expected) in test_cases {
            assert_eq!(S::possible_string_count(word.to_string()), expected);
        }
    }
}
