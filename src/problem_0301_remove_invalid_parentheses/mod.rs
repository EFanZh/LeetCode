pub mod backtracking;
pub mod backtracking_2;

pub trait Solution {
    fn remove_invalid_parentheses(s: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("()())()", &["(())()", "()()()"] as &[_]),
            ("(a)())()", &["(a())()", "(a)()()"]),
            (")(", &[""]),
        ];

        for (s, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::remove_invalid_parentheses(s.to_string())),
                expected
            );
        }
    }
}
