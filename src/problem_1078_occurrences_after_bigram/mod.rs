pub mod iterative;

pub trait Solution {
    fn find_ocurrences(text: String, first: String, second: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                ("alice is a good girl she is a good student", "a", "good"),
                &["girl", "student"] as &[_],
            ),
            (("we will we will rock you", "we", "will"), &["we", "rock"] as &[_]),
        ];

        for ((text, first, second), expected) in test_cases {
            assert_eq!(
                S::find_ocurrences(text.to_string(), first.to_string(), second.to_string()),
                expected,
            );
        }
    }
}
