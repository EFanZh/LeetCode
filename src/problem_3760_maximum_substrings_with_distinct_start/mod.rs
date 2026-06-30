pub mod greedy;

pub trait Solution {
    fn max_distinct(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abab", 2), ("abcd", 4), ("aaaa", 1)];

        for (s, expected) in test_cases {
            assert_eq!(S::max_distinct(s.to_string()), expected);
        }
    }
}
