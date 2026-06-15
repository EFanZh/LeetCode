pub mod iterative;

pub trait Solution {
    fn reverse_words(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("cat and mice", "cat dna mice"),
            ("book is nice", "book is ecin"),
            ("banana healthy", "banana healthy"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::reverse_words(s.to_string()), expected);
        }
    }
}
