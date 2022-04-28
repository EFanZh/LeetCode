pub mod iterative;

pub trait Solution {
    fn remove_outer_parentheses(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("(()())(())", "()()()"),
            ("(()())(())(()(()))", "()()()()(())"),
            ("()()", ""),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::remove_outer_parentheses(s.to_string()), expected);
        }
    }
}
