pub mod stack;

pub trait Solution {
    fn check_valid_string(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("()", true),
            ("(*)", true),
            ("(*))", true),
            ("(((((()*)(*)*))())())(()())())))((**)))))(()())()", false),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::check_valid_string(s.to_string()), expected);
        }
    }
}
