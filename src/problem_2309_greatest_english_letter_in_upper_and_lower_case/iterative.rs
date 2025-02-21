pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU64;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let state = s.bytes().fold(0, |state, c| state | (1 << (c - b'A')));

        NonZeroU64::new(state & (state >> (b'a' - b'A'))).map_or(String::new(), |state| {
            char::from(b'A' + 63 - state.leading_zeros() as u8).to_string()
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn greatest_letter(s: String) -> String {
        Self::greatest_letter(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
