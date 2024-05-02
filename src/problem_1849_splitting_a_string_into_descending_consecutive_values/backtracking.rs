pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::str::Bytes;

impl Solution {
    fn check(mut first: u8, mut iter: Bytes, mut expected: u64) -> bool {
        while expected != 0 {
            let mut value = u64::from(first - b'0');

            loop {
                match value.cmp(&expected) {
                    Ordering::Less => {
                        if let Some(next) = iter.next() {
                            value = value * 10 + u64::from(next - b'0');
                        } else {
                            return false;
                        }
                    }
                    Ordering::Equal => {
                        if let Some(next) = iter.next() {
                            first = next;
                            expected -= 1;

                            break;
                        }

                        return true;
                    }
                    Ordering::Greater => return false,
                }
            }
        }

        first == b'0' && iter.all(|c| c == b'0')
    }

    pub fn split_string(s: String) -> bool {
        let mut iter = s.bytes();

        let mut value = loop {
            if let Some(c) = iter.next() {
                if c != b'0' {
                    break u64::from(c - b'0');
                }
            } else {
                return false;
            }
        };

        while let Some(c) = iter.next() {
            if Self::check(c, iter.clone(), value - 1) {
                return true;
            }

            if let Some(next_value) = value
                .checked_mul(10)
                .and_then(|value| value.checked_add(u64::from(c - b'0')))
            {
                value = next_value;
            } else {
                break;
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn split_string(s: String) -> bool {
        Self::split_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
