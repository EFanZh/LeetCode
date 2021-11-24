pub mod iterative;
pub mod recursive;
pub mod recursive_2;

pub trait Solution {
    fn score_of_parentheses(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("()", 1), ("(())", 2), ("()()", 2), ("(()(()))", 6)];

        for (s, expected) in test_cases {
            assert_eq!(S::score_of_parentheses(s.to_string()), expected);
        }
    }
}
