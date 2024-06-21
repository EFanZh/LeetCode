pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str::Bytes;

impl Solution {
    #[inline(never)]
    fn start(mut iter: Bytes, result: i32) -> i32 {
        match iter.next() {
            None => result,
            Some(b'.') => Self::empty(iter, result),
            Some(_) => Self::hamster(iter, result),
        }
    }

    fn empty(mut iter: Bytes, result: i32) -> i32 {
        match iter.next() {
            None => result,
            Some(b'.') => Self::empty(iter, result),
            Some(_) => Self::empty_hamster(iter, result),
        }
    }

    fn hamster(mut iter: Bytes, result: i32) -> i32 {
        match iter.next() {
            Some(b'.') => Self::food(iter, result + 1),
            _ => -1,
        }
    }

    fn empty_hamster(mut iter: Bytes, mut result: i32) -> i32 {
        result += 1;

        match iter.next() {
            None => result,
            Some(b'.') => Self::food(iter, result),
            Some(_) => Self::hamster(iter, result),
        }
    }

    fn food(mut iter: Bytes, result: i32) -> i32 {
        match iter.next() {
            None => result,
            Some(b'.') => Self::empty(iter, result),
            Some(_) => Self::start(iter, result),
        }
    }

    pub fn minimum_buckets(hamsters: String) -> i32 {
        Self::start(hamsters.bytes(), 0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_buckets(hamsters: String) -> i32 {
        Self::minimum_buckets(hamsters)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
