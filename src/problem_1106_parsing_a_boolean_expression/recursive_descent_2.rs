pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ops::{BitAnd, BitOr};
use std::str::Bytes;

impl Solution {
    fn parse_application(input: &mut Bytes, mut f: impl FnMut(bool, bool) -> bool) -> bool {
        input.next();

        let mut result = Self::parse_expression(input);

        while let Some(b',') = input.next() {
            result = f(result, Self::parse_expression(input));
        }

        result
    }

    fn parse_expression(input: &mut Bytes) -> bool {
        match input.next().unwrap() {
            b't' => true,
            b'f' => false,
            b'!' => {
                input.next();

                let result = Self::parse_expression(input);

                input.next();

                !result
            }
            b'&' => Self::parse_application(input, bool::bitand),
            _ => Self::parse_application(input, bool::bitor),
        }
    }

    pub fn parse_bool_expr(expression: String) -> bool {
        Self::parse_expression(&mut expression.bytes())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn parse_bool_expr(expression: String) -> bool {
        Self::parse_bool_expr(expression)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
