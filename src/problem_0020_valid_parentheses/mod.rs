pub mod stack;
pub mod stack_2;

pub trait Solution {
    fn is_valid(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("()", true),
            ("()[]{}", true),
            ("(]", false),
            ("([)]", false),
            ("{[]}", true),
            ("(){}}{", false),
        ];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::is_valid(s.to_string()), expected);
        }
    }
}
