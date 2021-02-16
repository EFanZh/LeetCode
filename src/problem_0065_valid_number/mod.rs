pub mod dfa;

pub trait Solution {
    fn is_number(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("0", true),
            (" 0.1 ", true),
            ("abc", false),
            ("1 a", false),
            ("2e10", true),
            (" -90e3   ", true),
            (" 1e", false),
            ("e3", false),
            (" 6e-1", true),
            (" 99e2.5 ", false),
            ("53.5e93", true),
            (" --6 ", false),
            ("-+3", false),
            ("95a54e53", false),
            (".", false),
            (".1", true),
            ("+.e", false),
            ("0..", false),
            ("1e.", false),
            ("7.e-.", false),
            ("1E9", true),
        ];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::is_number(s.to_string()), expected);
        }
    }
}
