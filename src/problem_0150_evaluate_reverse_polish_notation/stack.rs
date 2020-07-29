pub struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for token in tokens {
            if let Ok(number) = token.parse() {
                stack.push(number);
            } else {
                let rhs = stack.pop().unwrap();
                let lhs = stack.last_mut().unwrap();

                match token.as_str() {
                    "+" => *lhs += rhs,
                    "-" => *lhs -= rhs,
                    "*" => *lhs *= rhs,
                    _ => *lhs /= rhs,
                }
            }
        }

        stack[0]
    }
}

impl super::Solution for Solution {
    fn eval_rpn(tokens: Vec<String>) -> i32 {
        Self::eval_rpn(tokens)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
