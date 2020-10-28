pub struct Solution;

use std::mem;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s = s.into_bytes();

        let mut iter = s.iter_mut().filter(|&&mut c| {
            const MAGIC_BITS: u64 = 1
                | 1 << (b'E' - b'A')
                | 1 << (b'I' - b'A')
                | 1 << (b'O' - b'A')
                | 1 << (b'U' - b'A')
                | 1 << (b'a' - b'A')
                | 1 << (b'e' - b'A')
                | 1 << (b'i' - b'A')
                | 1 << (b'o' - b'A')
                | 1 << (b'u' - b'A');

            let offset = c - b'A';

            offset <= b'u' - b'A' && (MAGIC_BITS >> offset) & 1 != 0
        });

        while let (Some(lhs), Some(rhs)) = (iter.next(), iter.next_back()) {
            mem::swap(lhs, rhs);
        }

        String::from_utf8(s).unwrap()
    }
}

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
