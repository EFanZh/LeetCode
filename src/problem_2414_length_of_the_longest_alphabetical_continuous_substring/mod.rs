pub mod greedy;

pub trait Solution {
    fn longest_continuous_substring(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abacaba", 2), ("abcde", 5)];

        for (s, expected) in test_cases {
            assert_eq!(S::longest_continuous_substring(s.to_string()), expected);
        }
    }
}
