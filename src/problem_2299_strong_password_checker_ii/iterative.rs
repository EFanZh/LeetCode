pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        password.len() >= 8 && {
            let mut state = 0_u8;
            let mut prev = 0;

            for c in password.bytes() {
                if c == prev {
                    return false;
                }

                state |= match c {
                    b'a'..=b'z' => 0b_0001,
                    b'A'..=b'Z' => 0b_0010,
                    b'0'..=b'9' => 0b_0100,
                    _ => 0b_1000,
                };

                prev = c;
            }

            state == 0b_1111
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn strong_password_checker_ii(password: String) -> bool {
        Self::strong_password_checker_ii(password)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
