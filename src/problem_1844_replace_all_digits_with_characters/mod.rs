pub mod iterative;

pub trait Solution {
    fn replace_digits(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("a1c1e1", "abcdef"), ("a1b2c3d4e", "abbdcfdhe")];

        for (s, expected) in test_cases {
            assert_eq!(S::replace_digits(s.to_string()), expected);
        }
    }
}
