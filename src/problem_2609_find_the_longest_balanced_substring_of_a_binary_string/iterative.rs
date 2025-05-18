pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut zero_length = 0_u8;
        let mut iter = s.bytes();
        let mut result = 0;

        'outer: while let Some(c) = iter.next() {
            if c == b'0' {
                zero_length += 1;
            } else {
                let mut one_length = 1;

                loop {
                    if let Some(c) = iter.next() {
                        if c == b'0' {
                            result = result.max(zero_length.min(one_length));
                            zero_length = 1;

                            break;
                        }

                        one_length += 1;
                    } else {
                        result = result.max(zero_length.min(one_length));

                        break 'outer;
                    }
                }
            }
        }

        i32::from(result * 2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_the_longest_balanced_substring(s: String) -> i32 {
        Self::find_the_longest_balanced_substring(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
