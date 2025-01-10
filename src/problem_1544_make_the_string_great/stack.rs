pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut s = s.into_bytes();
        let mut stack_length = 0_usize;
        let mut i = 0;

        while let Some(&c) = s.get(i) {
            if s.get(stack_length.wrapping_sub(1))
                .is_some_and(|&top| matches!(c.wrapping_sub(top), 32 | 224))
            {
                stack_length -= 1;
            } else {
                s[stack_length] = c;
                stack_length += 1;
            }

            i += 1;
        }

        s.truncate(stack_length);

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn make_good(s: String) -> String {
        Self::make_good(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
