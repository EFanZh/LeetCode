pub mod iterative;

pub trait Solution {
    fn fraction_addition(expression: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("-1/2+1/2", "0/1"),
            ("-1/2+1/2+1/3", "1/3"),
            ("1/3-1/2", "-1/6"),
            ("5/3+1/3", "2/1"),
            ("-5/2+10/3+7/9", "29/18"),
        ];

        for (expression, expected) in test_cases {
            assert_eq!(S::fraction_addition(expression.to_string()), expected);
        }
    }
}
