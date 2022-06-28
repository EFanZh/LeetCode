pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut brace_match = vec![0; s.len()];
        let mut stack = Vec::new();

        for (i, c) in s.bytes().enumerate() {
            match c {
                b'(' => stack.push(i),
                b')' => {
                    let j = stack.pop().unwrap();

                    brace_match[i] = j;
                    brace_match[j] = i;
                }
                _ => {}
            }
        }

        let s = s.into_bytes();
        let mut i = 0;
        let mut direction = 1;
        let mut result = String::new();

        while let Some(&c) = s.get(i) {
            if let b'(' | b')' = c {
                i = brace_match[i];
                direction = usize::wrapping_sub(0, direction);
            } else {
                result.push(char::from(c));
            }

            i = i.wrapping_add(direction);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_parentheses(s: String) -> String {
        Self::reverse_parentheses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
