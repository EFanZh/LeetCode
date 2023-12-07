pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut word = word;
        let ch = ch as u8;

        if let Some(i) = word.bytes().position(|c| c == ch) {
            let mut bytes = word.into_bytes();

            bytes[..=i].reverse();

            word = String::from_utf8(bytes).unwrap();
        }

        word
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_prefix(word: String, ch: char) -> String {
        Self::reverse_prefix(word, ch)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
