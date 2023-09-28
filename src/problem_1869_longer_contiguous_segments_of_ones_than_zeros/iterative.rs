pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut iter = s.bytes();
        let mut longest_zeros = 0_u8;
        let mut longest_ones = 0_u8;
        let mut length = 0_u8;

        'outer: loop {
            if let Some(c) = iter.next() {
                if c == b'0' {
                    length += 1;
                } else {
                    longest_zeros = longest_zeros.max(length);

                    length = 1;

                    loop {
                        if let Some(c) = iter.next() {
                            if c == b'0' {
                                longest_ones = longest_ones.max(length);

                                length = 1;

                                break;
                            }

                            length += 1;
                        } else {
                            longest_ones = longest_ones.max(length);

                            break 'outer;
                        }
                    }
                }
            } else {
                longest_zeros = longest_zeros.max(length);

                break;
            }
        }

        longest_ones > longest_zeros
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_zero_ones(s: String) -> bool {
        Self::check_zero_ones(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
