pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn resulting_string(s: String) -> String {
        let mut s = s.into_bytes();
        let mut retained = 0_usize;
        let mut i = 0;

        while let Some(&c) = s.get(i) {
            i += 1;

            if s.get(retained.wrapping_sub(1))
                .is_some_and(|&prev| matches!(c.abs_diff(prev), 1 | 25))
            {
                retained -= 1;
            } else {
                s[retained] = c;
                retained += 1;
            }
        }

        s.truncate(retained);

        String::try_from(s).ok().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn resulting_string(s: String) -> String {
        Self::resulting_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
