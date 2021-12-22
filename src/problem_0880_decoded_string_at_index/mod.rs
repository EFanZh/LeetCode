pub mod iterative;

pub trait Solution {
    fn decode_at_index(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("leet2code3", 10), 'o'),
            (("ha22", 5), 'h'),
            (("a2345678999999999999999", 1), 'a'),
            (("a23", 6), 'a'),
            (("a2b3c4d5e6f7g8h9", 3), 'b'),
        ];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::decode_at_index(s.to_string(), k), expected.to_string());
        }
    }
}
