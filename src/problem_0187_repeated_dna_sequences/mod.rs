pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn find_repeated_dna_sequences(s: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT",
                &["AAAAACCCCC", "CCCCCAAAAA"] as &[_],
            ),
            ("", &[]),
            ("AAAAAAAAAAA", &["AAAAAAAAAA"]),
            ("AAAAAAAAAAAAA", &["AAAAAAAAAA"]),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::find_repeated_dna_sequences(s.to_string()), expected);
        }
    }
}
