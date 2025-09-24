pub mod greedy;

pub trait Solution {
    fn last_non_empty_string(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aabcbbca", "ba"), ("abcd", "abcd")];

        for (s, expected) in test_cases {
            assert_eq!(S::last_non_empty_string(s.to_string()), expected);
        }
    }
}
