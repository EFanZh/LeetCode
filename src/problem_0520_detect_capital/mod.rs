pub mod iterative;
pub mod pattern_matching;

pub trait Solution {
    fn detect_capital_use(word: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("USA", true),
            ("FlaG", false),
            ("U", true),
            ("u", true),
            ("US", true),
            ("Us", true),
            ("uS", false),
            ("us", true),
            ("USB", true),
            ("USb", false),
            ("UsB", false),
            ("Usb", true),
            ("uSB", false),
            ("uSb", false),
            ("usB", false),
            ("usb", true),
        ];

        for (word, expected) in test_cases {
            assert_eq!(S::detect_capital_use(word.to_string()), expected);
        }
    }
}
