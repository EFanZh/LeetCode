pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str::Bytes;

impl Solution {
    fn helper(base: &mut Vec<u8>, mut iter: Bytes, result: &mut Vec<String>) {
        const DIFF: u8 = b'a' - b'A';

        if let Some(c) = iter.next() {
            base.push(c);

            Self::helper(base, iter.clone(), result);

            match c {
                b'A'..=b'Z' => {
                    *base.last_mut().unwrap() += DIFF;

                    Self::helper(base, iter, result);
                }
                b'a'..=b'z' => {
                    *base.last_mut().unwrap() -= DIFF;

                    Self::helper(base, iter, result);
                }
                _ => {}
            }

            base.pop();
        } else {
            result.push(String::from_utf8(base.clone()).unwrap());
        }
    }

    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut result = Vec::new();

        Self::helper(&mut Vec::with_capacity(s.len()), s.bytes(), &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn letter_case_permutation(s: String) -> Vec<String> {
        Self::letter_case_permutation(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
