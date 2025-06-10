pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let n = n as u16;
        let mut state = 0_u16;

        for mut value in [n, n * 2, n * 3] {
            loop {
                let digit = value % 10;

                value /= 10;

                if digit == 0 {
                    return false;
                }

                let probe = 1 << digit;

                if state & probe == 0 {
                    state |= probe;
                } else {
                    return false;
                }

                if value == 0 {
                    break;
                }
            }
        }

        state == ((1 << 9) - 1) << 1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_fascinating(n: i32) -> bool {
        Self::is_fascinating(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
