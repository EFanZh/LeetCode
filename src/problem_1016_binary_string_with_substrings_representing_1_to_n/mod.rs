pub mod brute_force;

pub trait Solution {
    fn query_string(s: String, n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("0110", 3), true),
            (("0110", 4), false),
            (("10010111100001110010", 10), false),
            (("011010101010111101010101011111111111111111111111111111111110000000000000011111101010101001010101010101010101010101111010101010111111111111111111111111111111111100000000000000111111010101010010101010101010101010100", 1_000_000_000),false)
        ];

        for ((s, n), expected) in test_cases {
            assert_eq!(S::query_string(s.to_string(), n), expected);
        }
    }
}
