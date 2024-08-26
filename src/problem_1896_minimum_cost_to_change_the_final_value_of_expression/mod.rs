pub mod recursive;

pub trait Solution {
    fn min_operations_to_flip(expression: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1&(0|1)", 1), ("(0&0)&(0&0&0)", 3), ("(0|(1|0&1))", 1)];

        for (expression, expected) in test_cases {
            assert_eq!(S::min_operations_to_flip(expression.to_string()), expected);
        }
    }
}
