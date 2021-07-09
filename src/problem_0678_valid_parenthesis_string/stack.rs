pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut min = 0_u32;
        let mut max = 0_u32;

        for c in s.bytes() {
            match c {
                b'(' => {
                    min += 1;
                    max += 1;
                }
                b')' => {
                    if max == 0 {
                        return false;
                    }

                    min = min.saturating_sub(1);
                    max -= 1;
                }
                _ => {
                    min = min.saturating_sub(1);
                    max += 1;
                }
            }
        }

        min == 0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_valid_string(s: String) -> bool {
        Self::check_valid_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
