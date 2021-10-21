pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    #[allow(clippy::if_then_some_else_none)]
    fn starts_with(first_digit: u8, s: &[u8], head: u64) -> Option<&[u8]> {
        if first_digit == 0 {
            if head == 0 {
                Some(s)
            } else {
                None
            }
        } else {
            let mut num = u64::from(first_digit);
            let mut iter = s.iter();
            let mut i = 0;

            loop {
                match num.cmp(&head) {
                    Ordering::Less => {
                        if let Some(&digit) = iter.next() {
                            num = num * 10 + u64::from(digit);
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    Ordering::Equal => return Some(&s[i..]),
                    Ordering::Greater => break,
                }
            }

            None
        }
    }

    fn is_additive(mut first: u64, mut second: u64, mut s: &[u8]) -> bool {
        while let Some((&digit, rest)) = s.split_first() {
            let third = first + second;

            if let Some(next) = Self::starts_with(digit, rest, third) {
                first = second;
                second = third;
                s = next;
            } else {
                return false;
            }
        }

        true
    }

    fn enumerate_head(s: &[u8], mut i: usize, mut f: impl FnMut(usize, u64) -> bool) -> bool {
        let (&first, s) = s.split_first().unwrap();
        let mut num = u64::from(first);

        if f(i, num) {
            return true;
        }

        if first != 0 {
            for &digit in s {
                i += 1;
                num = num * 10 + u64::from(digit);

                if f(i, num) {
                    return true;
                }
            }
        }

        false
    }

    pub fn is_additive_number(num: String) -> bool {
        if num.len() < 3 {
            false
        } else {
            let mut num = num.into_bytes();

            for digit in &mut num {
                *digit -= b'0';
            }

            let length = num.len();

            Self::enumerate_head(&num[..(length - 1) / 2], 1, |i, first| {
                let max_length_2 = if length < i * 3 {
                    length - i * 2
                } else {
                    (length - i) / 2
                };

                Self::enumerate_head(&num[i..i + max_length_2], i + 1, |j, second| {
                    Self::is_additive(first, second, &num[j..])
                })
            })
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_additive_number(num: String) -> bool {
        Self::is_additive_number(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
