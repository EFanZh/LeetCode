pub struct Solution;

use std::iter::Peekable;

impl Solution {
    fn whitespaces(iter: &mut Peekable<impl Iterator<Item = u8>>) {
        while let Some(b' ') = iter.peek() {
            iter.next();
        }
    }

    fn term(iter: &mut Peekable<impl Iterator<Item = u8>>) -> i32 {
        let c = iter.next().unwrap();

        if c == b'(' {
            Self::whitespaces(iter);

            let result = Self::expression(iter);

            Self::whitespaces(iter);

            iter.next();

            result
        } else {
            let mut num = i32::from(c - b'0');

            while let Some(&c @ b'0'..=b'9') = iter.peek() {
                num *= 10;
                num += i32::from(c - b'0');

                iter.next();
            }

            num
        }
    }

    fn expression(iter: &mut Peekable<impl Iterator<Item = u8>>) -> i32 {
        let mut result = Self::term(iter);

        loop {
            Self::whitespaces(iter);

            match iter.peek() {
                Some(b'+') => {
                    iter.next();

                    Self::whitespaces(iter);

                    result += Self::term(iter);
                }
                Some(b'-') => {
                    iter.next();

                    Self::whitespaces(iter);

                    result -= Self::term(iter);
                }
                _ => break,
            }
        }

        result
    }

    pub fn calculate(s: String) -> i32 {
        let mut iter = s.bytes().peekable();

        Self::whitespaces(&mut iter);
        Self::expression(&mut iter)
    }
}

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
