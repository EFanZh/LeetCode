pub mod iterative;

pub trait Solution {
    fn has_all_codes(s: String, k: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("00110110", 2), true),
            (("0110", 1), true),
            (("0110", 2), false),
            (("00000000001011100", 3), true),
        ];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::has_all_codes(s.to_string(), k), expected);
        }
    }
}
