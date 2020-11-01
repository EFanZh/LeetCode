pub struct Solution;

use std::collections::HashSet;
use std::slice::Iter;

impl Solution {
    fn helper(
        mut iter: Iter<u8>,
        stack: usize,
        extra_left: usize,
        extra_right: usize,
        base: &mut String,
        result: &mut HashSet<String>,
    ) {
        if let Some(&c) = iter.next() {
            match c {
                b'(' => {
                    base.push('(');

                    Self::helper(iter.clone(), stack + 1, extra_left, extra_right, base, result);

                    base.pop();

                    if extra_left != 0 {
                        Self::helper(iter.clone(), stack, extra_left - 1, extra_right, base, result);
                    }
                }
                b')' => {
                    if stack != 0 {
                        base.push(')');

                        Self::helper(iter.clone(), stack - 1, extra_left, extra_right, base, result);

                        base.pop();
                    }

                    if extra_right != 0 {
                        Self::helper(iter.clone(), stack, extra_left, extra_right - 1, base, result);
                    }
                }
                _ => {
                    base.push(char::from(c));

                    Self::helper(iter.clone(), stack, extra_left, extra_right, base, result);

                    base.pop();
                }
            }
        } else if stack == 0 {
            result.insert(base.clone());
        }
    }

    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut extra_left = 0;
        let mut extra_right = 0;

        for c in s.bytes() {
            match c {
                b'(' => extra_left += 1,
                b')' => {
                    if extra_left == 0 {
                        extra_right += 1;
                    } else {
                        extra_left -= 1;
                    }
                }
                _ => {}
            }
        }

        let mut result = HashSet::new();

        Self::helper(
            s.as_bytes().iter(),
            0,
            extra_left,
            extra_right,
            &mut String::with_capacity(s.len()),
            &mut result,
        );

        result.into_iter().collect()
    }
}

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
