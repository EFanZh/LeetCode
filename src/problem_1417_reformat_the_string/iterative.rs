pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reformat(s: String) -> String {
        let mut s = s;
        let mut digits = 0;
        let mut letters = 0;

        for c in s.bytes() {
            if c.is_ascii_digit() {
                digits += 1;
            } else {
                letters += 1;
            }
        }

        let (digit_start, letter_start) = match letters - digits {
            -1 | 0 => (0, 1),
            1 => (1, 0),
            _ => {
                s.clear();

                return s;
            }
        };

        let mut s = s.into_bytes();
        let mut i = digit_start;
        let mut j = letter_start;

        while let Some(&should_be_digit) = s.get(i) {
            if !should_be_digit.is_ascii_digit() {
                while !s[j].is_ascii_digit() {
                    j += 2;
                }

                s.swap(i, j);
                j += 2;
            }

            i += 2;
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reformat(s: String) -> String {
        Self::reformat(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
