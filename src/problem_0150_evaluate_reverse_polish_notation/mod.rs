pub mod stack;

pub trait Solution {
    fn eval_rpn(tokens: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["2", "1", "+", "3", "*"] as &[_], 9),
            (&["4", "13", "5", "/", "+"], 6),
            (
                &["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"],
                22,
            ),
            (&["4", "3", "-"], 1),
        ];

        for (tokens, expected) in test_cases {
            assert_eq!(
                S::eval_rpn(tokens.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
