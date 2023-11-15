pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut s = s.into_bytes();
        let slice = s.as_mut_slice();
        let mut writer = 0;
        let mut i = 0;
        let mut prev = 0;
        let mut count = 0_u8;

        while let Some(&c) = slice.get(i) {
            i += 1;

            if c == prev {
                if count < 2 {
                    count += 1;
                } else {
                    continue;
                }
            } else {
                prev = c;
                count = 1;
            }

            slice[writer] = c;
            writer += 1;
        }

        s.truncate(writer);

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn make_fancy_string(s: String) -> String {
        Self::make_fancy_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
