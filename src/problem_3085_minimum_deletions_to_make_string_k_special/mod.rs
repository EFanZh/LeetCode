pub mod greedy;

pub trait Solution {
    fn minimum_deletions(word: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("aabcaba", 0), 3), (("dabdcbdcdcd", 2), 2), (("aaabaaa", 2), 1)];

        for ((word, k), expected) in test_cases {
            assert_eq!(S::minimum_deletions(word.to_string(), k), expected);
        }
    }
}
