pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    fn get_max_length<I: Iterator<Item = u8>>(iter: I, key: u8) -> i32 {
        let mut max_length = 0;
        let mut left = 0;
        let mut right = 0;

        for c in iter {
            if c == key {
                left += 1;
            } else {
                right += 1;
            }

            match left.cmp(&right) {
                Ordering::Less => {
                    left = 0;
                    right = 0;
                }
                Ordering::Equal => max_length = max_length.max(left + right),
                Ordering::Greater => {}
            }
        }

        max_length
    }

    pub fn longest_valid_parentheses(s: String) -> i32 {
        let length_1 = Self::get_max_length(s.bytes(), b'(');
        let length_2 = Self::get_max_length(s.into_bytes().into_iter().rev(), b')');

        length_1.max(length_2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_valid_parentheses(s: String) -> i32 {
        Self::longest_valid_parentheses(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
