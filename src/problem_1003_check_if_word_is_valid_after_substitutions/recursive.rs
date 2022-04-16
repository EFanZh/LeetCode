pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str::Bytes;

impl Solution {
    fn helper(iter: &mut Bytes, tail: Option<u8>) -> bool {
        loop {
            match iter.next() {
                Some(b'a') => {
                    if !Self::helper(iter, Some(b'b')) || !Self::helper(iter, Some(b'c')) {
                        return false;
                    }
                }
                value => return value == tail,
            }
        }
    }

    pub fn is_valid(s: String) -> bool {
        Self::helper(&mut s.bytes(), None)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_valid(s: String) -> bool {
        Self::is_valid(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
