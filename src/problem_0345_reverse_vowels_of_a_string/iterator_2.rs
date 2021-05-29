pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.into_bytes();

        let mut iter = s
            .iter_mut()
            .filter(|&&mut c| matches!(c, b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u'));

        while let (Some(lhs), Some(rhs)) = (iter.next(), iter.next_back()) {
            mem::swap(lhs, rhs);
        }

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_vowels(s: String) -> String {
        Self::reverse_vowels(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
