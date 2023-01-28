pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut s = s.into_bytes();
        let s_length = s.len();
        let mut result_length = 0;
        let mut i = 0;

        while i < s_length {
            i += 1;

            let mut stack = 1;

            loop {
                let c = s[i];

                i += 1;

                if c == b'(' {
                    stack += 1;

                    s[result_length] = b'(';
                } else {
                    stack -= 1;

                    if stack == 0 {
                        break;
                    }

                    s[result_length] = b')';
                }

                result_length += 1;
            }
        }

        s.truncate(result_length);

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_outer_parentheses(s: String) -> String {
        Self::remove_outer_parentheses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
