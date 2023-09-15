pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut s = s.into_bytes();
        let mut i = 0;

        while let Some([left, right]) = s.get_mut(i..i + 2) {
            *right = *left + (*right - b'0');

            i += 2;
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn replace_digits(s: String) -> String {
        Self::replace_digits(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
