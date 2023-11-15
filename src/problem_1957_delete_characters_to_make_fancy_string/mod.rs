pub mod iterative;

pub trait Solution {
    fn make_fancy_string(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("leeetcode", "leetcode"), ("aaabaaaa", "aabaa"), ("aab", "aab")];

        for (s, expected) in test_cases {
            assert_eq!(S::make_fancy_string(s.to_string()), expected);
        }
    }
}
