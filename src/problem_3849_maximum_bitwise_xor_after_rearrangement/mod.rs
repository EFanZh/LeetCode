pub mod greedy;

pub trait Solution {
    fn maximum_xor(s: String, t: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("101", "011"), "110"),
            (("0110", "1110"), "1101"),
            (("0101", "1001"), "1111"),
        ];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::maximum_xor(s.to_string(), t.to_string()), expected);
        }
    }
}
