pub mod brute_force;
pub trait Solution {
    fn my_atoi(str: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [
            ("42", 42),
            ("   -42", -42),
            ("4193 with words", 4193),
            ("words and 987", 0),
            ("-91283472332", -2_147_483_648),
        ];

        for (str, expected) in test_cases.iter().copied() {
            assert_eq!(S::my_atoi(str.to_owned()), expected);
        }
    }
}
