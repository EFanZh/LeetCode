pub mod iterative;

pub trait Solution {
    fn last_substring(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abab", "bab"), ("leetcode", "tcode"), ("aaaaaaab", "b")];

        for (s, expected) in test_cases {
            assert_eq!(S::last_substring(s.to_string()), expected);
        }
    }
}
