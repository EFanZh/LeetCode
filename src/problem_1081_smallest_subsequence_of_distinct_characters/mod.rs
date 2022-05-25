pub mod stack;

pub trait Solution {
    fn smallest_subsequence(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("bcabc", "abc"), ("cbacdcbc", "acdb")];

        for (s, expected) in test_cases {
            assert_eq!(S::smallest_subsequence(s.to_string()), expected);
        }
    }
}
