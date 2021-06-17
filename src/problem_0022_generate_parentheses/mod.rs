pub mod backtracking;
pub mod backtracking_2;
pub mod backtracking_3;

pub trait Solution {
    fn generate_parenthesis(n: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, &[""] as &[_]),
            (1, &["()"]),
            (2, &["(())", "()()"]),
            (3, &["((()))", "(()())", "(())()", "()(())", "()()()"]),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::generate_parenthesis(n), expected);
        }
    }
}
