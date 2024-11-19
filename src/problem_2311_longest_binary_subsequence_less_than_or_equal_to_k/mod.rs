pub mod greedy;

pub trait Solution {
    fn longest_subsequence(s: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("1001010", 5), 5),
            (("00101001", 1), 6),
            (
                (
                    "101100011000010101011100011111111001011101000101000010001100101110010010011000100010001110011111000100100101110000100010010010100010",
                    978_095_074,
                ),
                84,
            ),
            (("0", 583_196_182), 1),
        ];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::longest_subsequence(s.to_string(), k), expected);
        }
    }
}
