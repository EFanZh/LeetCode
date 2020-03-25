pub mod stack;

pub trait Solution {
    fn is_valid(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [
            ("()", true),
            ("()[]{}", true),
            ("(]", false),
            ("([)]", false),
            ("{[]}", true),
        ];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::is_valid(s.to_owned()), expected);
        }
    }
}
