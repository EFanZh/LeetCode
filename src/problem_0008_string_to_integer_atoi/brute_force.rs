pub struct Solution {}

use std::vec;

impl Solution {
    fn parse_positive(first_char: u8, iter: vec::IntoIter<u8>) -> i32 {
        let mut result = i32::from(first_char - b'0');

        for c in iter {
            if c.is_ascii_digit() {
                if let Some(new_result) = result.checked_mul(10).and_then(|r| r.checked_add(i32::from(c - b'0'))) {
                    result = new_result;
                } else {
                    return i32::max_value();
                }
            } else {
                break;
            }
        }

        result
    }

    fn parse_negaitive(mut iter: vec::IntoIter<u8>) -> i32 {
        for c in iter.by_ref() {
            match c {
                b'0' => continue,
                b'1'..=b'9' => {
                    let mut result = -(i32::from(c - b'0'));

                    for c in iter {
                        if c.is_ascii_digit() {
                            if let Some(new_result) =
                                result.checked_mul(10).and_then(|r| r.checked_sub(i32::from(c - b'0')))
                            {
                                result = new_result;
                            } else {
                                return i32::min_value();
                            }
                        } else {
                            break;
                        }
                    }

                    return result;
                }
                _ => return 0,
            }
        }

        0
    }

    pub fn my_atoi(str: String) -> i32 {
        // \s*[+-]?\d+

        let mut iter = str.into_bytes().into_iter();

        for c in iter.by_ref() {
            match c {
                b'+' => match iter.next() {
                    Some(c) if c.is_ascii_digit() => return Self::parse_positive(c, iter),
                    _ => break,
                },
                b'-' => return Self::parse_negaitive(iter),
                b'0'..=b'9' => return Self::parse_positive(c, iter),
                _ => {
                    if !c.is_ascii_whitespace() {
                        break;
                    }
                }
            }
        }

        0
    }
}

impl super::Solution for Solution {
    fn my_atoi(str: String) -> i32 {
        Self::my_atoi(str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
