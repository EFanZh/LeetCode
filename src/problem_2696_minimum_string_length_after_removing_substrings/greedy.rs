pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut s = s.into_bytes();
        let s = s.as_mut_slice();
        let mut stack_length = 0_usize;
        let mut i = 0;

        while let Some(&c) = s.get(i) {
            i += 1;

            match c {
                b'B' => {
                    if s.get(stack_length.wrapping_sub(1)).copied() == Some(b'A') {
                        stack_length -= 1;

                        continue;
                    }
                }
                b'D' => {
                    if s.get(stack_length.wrapping_sub(1)).copied() == Some(b'C') {
                        stack_length -= 1;

                        continue;
                    }
                }
                _ => {}
            }

            s[stack_length] = c;
            stack_length += 1;
        }

        stack_length as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_length(s: String) -> i32 {
        Self::min_length(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
