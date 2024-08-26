pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str::Bytes;

impl Solution {
    fn parse_term(iter: &mut Bytes) -> (bool, u32) {
        match iter.next().unwrap() {
            b'(' => Self::parse_expression(iter),
            c => (c != b'0', 1),
        }
    }

    fn parse_expression(iter: &mut Bytes) -> (bool, u32) {
        let mut lhs = Self::parse_term(iter);

        while let Some(operator) = iter.next() {
            if operator == b')' {
                break;
            }

            let rhs = Self::parse_term(iter);
            let is_or = operator == b'|';

            lhs = match (lhs.0 == is_or, rhs.0 == is_or) {
                (false, false) => (!is_or, lhs.1.min(rhs.1)),
                (true, false) | (false, true) => (is_or, 1),
                (true, true) => (is_or, lhs.1.min(rhs.1) + 1),
            };
        }

        lhs
    }

    pub fn min_operations_to_flip(expression: String) -> i32 {
        Self::parse_expression(&mut expression.bytes()).1 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations_to_flip(expression: String) -> i32 {
        Self::min_operations_to_flip(expression)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
