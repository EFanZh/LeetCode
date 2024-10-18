pub mod iterative;

pub trait Solution {
    fn rearrange_characters(s: String, target: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("ilovecodingonleetcode", "code"), 2),
            (("abcba", "abc"), 1),
            (("abbaccaddaeea", "aaaaa"), 1),
        ];

        for ((s, target), expected) in test_cases {
            assert_eq!(S::rearrange_characters(s.to_string(), target.to_string()), expected);
        }
    }
}
