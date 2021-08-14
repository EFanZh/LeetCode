pub mod recursive_descent;

pub trait Solution {
    fn evaluate(expression: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("(let x 2 (mult x (let x 3 y 4 (add x y))))", 14),
            ("(let x 3 x 2 x)", 2),
            ("(let x 1 y 2 x (add x y) (add x y))", 5),
            ("(let x 2 (add (let x 3 (let x 4 x)) x))", 6),
            ("(let a1 3 b2 (add a1 1) b2)", 4),
            ("(let x 7 -12)", -12),
        ];

        for (expression, expected) in test_cases {
            assert_eq!(S::evaluate(expression.to_string()), expected);
        }
    }
}
