pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        match word.as_bytes() {
            [_] => true,
            [b'A'..=b'Z', b'A'..=b'Z', rest @ ..] => rest.iter().all(u8::is_ascii_uppercase),
            [b'A'..=b'Z', b'a'..=b'z', rest @ ..] => rest.iter().all(u8::is_ascii_lowercase),
            word => word.iter().all(u8::is_ascii_lowercase),
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn detect_capital_use(word: String) -> bool {
        Self::detect_capital_use(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
