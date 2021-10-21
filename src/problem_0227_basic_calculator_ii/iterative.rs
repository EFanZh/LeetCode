pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter::Peekable;
use std::mem;

impl Solution {
    fn whitespaces(iter: &mut Peekable<impl Iterator<Item = u8>>) {
        while let Some(b' ') = iter.peek() {
            iter.next();
        }
    }

    fn op(iter: &mut Peekable<impl Iterator<Item = u8>>) -> Option<u8> {
        Self::whitespaces(iter);

        iter.next()
    }

    fn number(iter: &mut Peekable<impl Iterator<Item = u8>>) -> i32 {
        Self::whitespaces(iter);

        let mut num = i32::from(iter.next().unwrap() - b'0');

        while let Some(&c @ b'0'..=b'9') = iter.peek() {
            num *= 10;
            num += i32::from(c - b'0');

            iter.next();
        }

        num
    }

    #[allow(clippy::unnested_or_patterns)]
    pub fn calculate(s: String) -> i32 {
        let mut iter = s.bytes().peekable();
        let mut first = Self::number(&mut iter);

        let (mut op, mut second) = loop {
            match Self::op(&mut iter) {
                Some(op @ b'+') | Some(op @ b'-') => break (op, Self::number(&mut iter)),
                Some(b'*') => first *= Self::number(&mut iter),
                Some(b'/') => first /= Self::number(&mut iter),
                _ => return first,
            }
        };

        loop {
            match Self::op(&mut iter) {
                Some(op_2 @ b'+') | Some(op_2 @ b'-') => {
                    if mem::replace(&mut op, op_2) == b'+' {
                        first += mem::replace(&mut second, Self::number(&mut iter));
                    } else {
                        first -= mem::replace(&mut second, Self::number(&mut iter));
                    }
                }
                Some(b'*') => second *= Self::number(&mut iter),
                Some(b'/') => second /= Self::number(&mut iter),
                _ => break,
            }
        }

        if op == b'+' {
            first + second
        } else {
            first - second
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn calculate(s: String) -> i32 {
        Self::calculate(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
