pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut lhs = 0;
        let mut rhs = 0;
        let mut sign = 1;

        for c in s.bytes() {
            match c {
                b'+' => {
                    lhs += rhs * sign;
                    rhs = 0;
                    sign = 1;
                }
                b'-' => {
                    lhs += rhs * sign;
                    rhs = 0;
                    sign = -1;
                }
                b'(' => {
                    stack.push((lhs, sign));
                    lhs = 0;
                    sign = 1;
                }
                b')' => {
                    rhs = lhs + rhs * sign;

                    let (saved_lhs, saved_sign) = stack.pop().unwrap();

                    lhs = saved_lhs;
                    sign = saved_sign;
                }
                b'0'..=b'9' => rhs = rhs * 10 + i32::from(c - b'0'),
                _ => {}
            }
        }

        lhs + rhs * sign
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn calculate(s: String) -> i32 {
        Self::calculate(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
