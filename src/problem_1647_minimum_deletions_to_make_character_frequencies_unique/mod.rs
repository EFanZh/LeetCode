pub mod iterative;

pub trait Solution {
    fn min_deletions(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("aab", 0), ("aaabbbcc", 2), ("ceabaacb", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::min_deletions(s.to_string()), expected);
        }
    }
}
