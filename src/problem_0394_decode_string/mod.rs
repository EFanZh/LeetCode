pub mod recursive;
pub mod stack;
pub mod stack_2;

pub trait Solution {
    fn decode_string(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("3[a]2[bc]", "aaabcbc"),
            ("3[a2[c]]", "accaccacc"),
            ("2[abc]3[cd]ef", "abcabccdcdcdef"),
            ("abc3[cd]xyz", "abccdcdcdxyz"),
            ("10[abc]", "abcabcabcabcabcabcabcabcabcabc"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::decode_string(s.to_string()), expected);
        }
    }
}
