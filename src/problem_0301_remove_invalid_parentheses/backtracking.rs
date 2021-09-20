pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    fn helper(s: &[u8], base: &mut String, stack: usize, result: &mut HashSet<String>, max_length: &mut usize) {
        if let Some((&first, rest)) = s.split_first() {
            match first {
                b'(' => {
                    base.push('(');

                    Self::helper(rest, base, stack + 1, result, max_length);

                    base.pop();

                    if base.len() + s.len() > *max_length {
                        Self::helper(rest, base, stack, result, max_length);
                    }
                }
                b')' => {
                    if stack != 0 {
                        base.push(')');

                        Self::helper(rest, base, stack - 1, result, max_length);

                        base.pop();
                    }

                    if base.len() + s.len() > *max_length {
                        Self::helper(rest, base, stack, result, max_length);
                    }
                }
                _ => {
                    base.push(char::from(first));

                    Self::helper(rest, base, stack, result, max_length);

                    base.pop();
                }
            }
        } else if stack == 0 {
            if base.len() == *max_length {
                result.insert(base.clone());
            } else {
                result.clear();
                result.insert(base.clone());
                *max_length = base.len();
            }
        }
    }

    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut result = HashSet::new();

        Self::helper(
            s.as_bytes(),
            &mut String::with_capacity(s.len()),
            0,
            &mut result,
            &mut 0,
        );

        result.into_iter().collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_invalid_parentheses(s: String) -> Vec<String> {
        Self::remove_invalid_parentheses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
