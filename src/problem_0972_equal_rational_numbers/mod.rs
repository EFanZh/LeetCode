pub mod convert_to_fraction;

pub trait Solution {
    fn is_rational_equal(s: String, t: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("0.(52)", "0.5(25)"), true),
            (("0.1666(6)", "0.166(66)"), true),
            (("0.9(9)", "1."), true),
            (("1.0", "1"), true),
        ];

        for ((s, t), expected) in test_cases {
            assert_eq!(S::is_rational_equal(s.to_string(), t.to_string()), expected);
        }
    }
}
