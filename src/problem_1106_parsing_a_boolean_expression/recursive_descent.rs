pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::ops::{BitAnd, BitOr};

impl Solution {
    fn parse_application(input: &[u8], mut f: impl FnMut(bool, bool) -> bool) -> (bool, &[u8]) {
        let (mut result, mut rest) = Self::parse_expression(&input[1..]);

        loop {
            let (&first, new_rest) = rest.split_first().unwrap();

            if first == b',' {
                let (value, new_rest) = Self::parse_expression(new_rest);

                result = f(result, value);
                rest = new_rest;
            } else {
                return (result, new_rest);
            }
        }
    }

    fn parse_expression(input: &[u8]) -> (bool, &[u8]) {
        let (&first, rest) = input.split_first().unwrap();

        match first {
            b't' => (true, rest),
            b'f' => (false, rest),
            b'!' => {
                let (result, rest) = Self::parse_expression(&rest[1..]);

                (!result, &rest[1..])
            }
            b'&' => Self::parse_application(rest, bool::bitand),
            _ => Self::parse_application(rest, bool::bitor),
        }
    }

    pub fn parse_bool_expr(expression: String) -> bool {
        Self::parse_expression(expression.as_bytes()).0
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
