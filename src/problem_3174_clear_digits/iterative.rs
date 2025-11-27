pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut s = s.into_bytes();
        let mut retained = 0;

        for i in 0..s.len() {
            let c = s[i];

            if c.is_ascii_digit() {
                retained -= 1;
            } else {
                s[retained] = c;
                retained += 1;
            }
        }

        s.truncate(retained);

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn clear_digits(s: String) -> String {
        Self::clear_digits(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
