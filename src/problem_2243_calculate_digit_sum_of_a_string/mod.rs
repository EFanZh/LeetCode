pub mod iterative;

pub trait Solution {
    fn digit_sum(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("11111222223", 3), "135"), (("00000000", 3), "000")];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::digit_sum(s.to_string(), k), expected.to_string());
        }
    }
}
