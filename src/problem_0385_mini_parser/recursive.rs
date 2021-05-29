use super::super::data_structures::NestedInteger;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter::Peekable;

impl Solution {
    fn integer(iter: &mut Peekable<impl Iterator<Item = u8>>) -> Option<i32> {
        fn build_integer(iter: &mut Peekable<impl Iterator<Item = u8>>) -> i32 {
            let mut result = i32::from(iter.next().unwrap() - b'0');

            while let Some(&c @ b'0'..=b'9') = iter.peek() {
                result = result * 10 + i32::from(c - b'0');
                iter.next();
            }

            result
        }

        match iter.peek() {
            Some(b'0'..=b'9') => Some(build_integer(iter)),
            Some(b'-') => {
                iter.next();

                Some(-build_integer(iter))
            }
            _ => None,
        }
    }

    fn list(iter: &mut Peekable<impl Iterator<Item = u8>>) -> Vec<NestedInteger> {
        let mut result = Vec::new();

        if let Some(item) = Self::nested_integer(iter) {
            result.push(item);

            while let Some(b',') = iter.peek() {
                iter.next();
                result.push(Self::nested_integer(iter).unwrap());
            }
        }

        result
    }

    fn nested_integer(iter: &mut Peekable<impl Iterator<Item = u8>>) -> Option<NestedInteger> {
        if let Some(b'[') = iter.peek() {
            iter.next();

            let list = Self::list(iter);

            iter.next();

            Some(NestedInteger::List(list))
        } else {
            Self::integer(iter).map(NestedInteger::Int)
        }
    }

    pub fn deserialize(s: String) -> NestedInteger {
        Self::nested_integer(&mut s.bytes().peekable()).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn deserialize(s: String) -> NestedInteger {
        Self::deserialize(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
