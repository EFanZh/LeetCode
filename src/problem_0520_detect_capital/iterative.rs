pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut iter = word.as_bytes().iter();

        if iter.next().unwrap().is_ascii_uppercase() {
            iter.next().map_or(true, |second| {
                if second.is_ascii_uppercase() {
                    iter.all(u8::is_ascii_uppercase)
                } else {
                    iter.all(u8::is_ascii_lowercase)
                }
            })
        } else {
            iter.all(u8::is_ascii_lowercase)
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
