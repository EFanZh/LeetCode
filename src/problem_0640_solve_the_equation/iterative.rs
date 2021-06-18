pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse_number(input: &[u8]) -> Option<(i32, &[u8])> {
        let (first, mut rest) = input.split_first()?;

        if first.is_ascii_digit() {
            let mut result = i32::from(first - b'0');

            while let Some((c @ b'0'..=b'9', next_rest)) = rest.split_first() {
                result = result * 10 + i32::from(c - b'0');
                rest = next_rest;
            }

            Some((result, rest))
        } else {
            None
        }
    }

    fn parse_term(input: &[u8]) -> ((i32, i32), &[u8]) {
        if let Some((number, rest)) = Self::parse_number(input) {
            if let Some((b'x', rest)) = rest.split_first() {
                ((number, 0), rest)
            } else {
                ((0, number), rest)
            }
        } else {
            ((1, 0), &input[1..])
        }
    }

    fn parse_one_side(input: &[u8]) -> ((i32, i32), &[u8]) {
        let ((mut x_factor, mut constant), mut input) = if let Some((b'-', rest)) = input.split_first() {
            let ((x, n), rest) = Self::parse_term(rest);

            ((-x, -n), rest)
        } else {
            Self::parse_term(input)
        };

        while let Some((&op, rest)) = input.split_first().filter(|(&op, _)| op == b'+' || op == b'-') {
            let ((x, n), rest) = Self::parse_term(rest);

            if op == b'+' {
                x_factor += x;
                constant += n;
            } else {
                x_factor -= x;
                constant -= n;
            }

            input = rest;
        }

        ((x_factor, constant), input)
    }

    pub fn solve_equation(mut equation: String) -> String {
        use std::fmt::Write;

        let ((lhs_x, lhs_n), input) = Self::parse_one_side(equation.as_bytes());
        let ((rhs_x, rhs_n), _) = Self::parse_one_side(&input[1..]);
        let total_x = lhs_x - rhs_x;
        let total_n = rhs_n - lhs_n;

        equation.clear();

        if total_x == 0 {
            equation.push_str(if total_n == 0 {
                "Infinite solutions"
            } else {
                "No solution"
            });
        } else {
            write!(&mut equation, "x={}", total_n / total_x).unwrap();
        }

        equation
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn solve_equation(equation: String) -> String {
        Self::solve_equation(equation)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
