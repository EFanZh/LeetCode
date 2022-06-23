pub mod recursive_descent;
pub mod recursive_descent_2;

pub trait Solution {
    fn parse_bool_expr(expression: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("!(f)", true), ("|(f,t)", true), ("&(t,f)", false)];

        for (expression, expected) in test_cases {
            assert_eq!(S::parse_bool_expr(expression.to_string()), expected);
        }
    }
}
