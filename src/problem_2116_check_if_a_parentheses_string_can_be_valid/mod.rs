pub mod two_passes;

pub trait Solution {
    fn can_be_valid(s: String, locked: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("))()))", "010100"), true),
            (("()()", "0000"), true),
            ((")", "0"), false),
            (("())))(()())(()()(((()", "000000000000000000000"), false),
            (("()", "11"), true),
            (
                (
                    "())(()(()(())()())(())((())(()())((())))))(((((((())(()))))(",
                    "100011110110011011010111100111011101111110000101001101001111",
                ),
                false,
            ),
            (
                (
                    "))))(())((()))))((()((((((())())((()))((((())()()))(()",
                    "101100101111110000000101000101001010110001110000000101",
                ),
                false,
            ),
            (("((", "11"), false),
            (
                (
                    "())()))()(()(((())(()()))))((((()())(())",
                    "1011101100010001001011000000110010100101",
                ),
                true,
            ),
        ];

        for ((s, locked), expected) in test_cases {
            assert_eq!(S::can_be_valid(s.to_string(), locked.to_string()), expected);
        }
    }
}
