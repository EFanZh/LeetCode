pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_valid_substrings(s: String) -> i64 {
        let mut expected = b'a';
        let mut length = 0;
        let mut result = 0;

        for c in s.bytes() {
            'block: {
                if c == expected {
                    if c == b'c' {
                        expected = b'a';
                        length = 2;

                        break 'block;
                    }

                    expected += 1;
                } else {
                    expected = if c == b'a' { b'b' } else { b'a' };
                }

                length += 1;
            }

            result += length;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_valid_substrings(s: String) -> i64 {
        Self::count_valid_substrings(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
