pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse_sign(input: &[u8]) -> (bool, &[u8]) {
        match input.split_first() {
            Some((b'+', rest)) => (true, rest),
            Some((b'-', rest)) => (false, rest),
            _ => (true, input),
        }
    }

    fn parse_number(input: &[u8]) -> Option<(u8, &[u8])> {
        let (first, rest_1) = input.split_first()?;
        let first_digit = first - b'0';

        Some(rest_1.split_first().map_or((first_digit, rest_1), |(&second, rest_2)| {
            if second == b'0' {
                (first_digit * 10, rest_2)
            } else {
                (first_digit, rest_1)
            }
        }))
    }

    fn parse_fraction(input: &[u8]) -> Option<(i16, &[u8])> {
        let (is_positive, input) = Self::parse_sign(input);
        let (numerator, input) = Self::parse_number(input)?;
        let (denominator, input) = Self::parse_number(&input[1..]).unwrap();

        let numerator = if is_positive {
            i16::from(numerator)
        } else {
            -i16::from(numerator)
        };

        Some((numerator * (2520 / i16::from(denominator)), input))
    }

    fn gcd(mut x: i32, mut y: i32) -> i32 {
        while y != 0 {
            let z = x % y;

            x = y;
            y = z;
        }

        x
    }

    fn parse_expression(mut input: &[u8]) -> (i32, i32) {
        let mut sum = 0;

        while let Some((num, suffix)) = Self::parse_fraction(input) {
            sum += i32::from(num);
            input = suffix;
        }

        let gcd = Self::gcd(sum.abs(), 2520);

        (sum / gcd, 2520 / gcd)
    }

    pub fn fraction_addition(expression: String) -> String {
        use std::fmt::Write;

        let mut expression = expression;
        let (numerator, denominator) = Self::parse_expression(expression.as_bytes());

        expression.clear();

        write!(expression, "{}/{}", numerator, denominator).unwrap();

        expression
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn fraction_addition(expression: String) -> String {
        Self::fraction_addition(expression)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
