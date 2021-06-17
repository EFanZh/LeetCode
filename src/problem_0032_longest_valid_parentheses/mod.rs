pub mod bidirectional_scan;
pub mod dynamic_programming;
pub mod stack;

pub trait Solution {
    fn longest_valid_parentheses(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("(()", 2), (")()())", 4), ("()", 2), ("()(()", 2), (")(", 0)];

        for (s, expected) in test_cases {
            assert_eq!(S::longest_valid_parentheses(s.to_string()), expected);
        }
    }
}
