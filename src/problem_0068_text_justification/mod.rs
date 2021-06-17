pub mod iterative;
pub mod recursive;

pub trait Solution {
    fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["This", "is", "an", "example", "of", "text", "justification."] as &[_],
                    16,
                ),
                &["This    is    an", "example  of text", "justification.  "] as &[_],
            ),
            (
                (&["What", "must", "be", "acknowledgment", "shall", "be"], 16),
                &["What   must   be", "acknowledgment  ", "shall be        "],
            ),
            (
                (
                    &[
                        "Science",
                        "is",
                        "what",
                        "we",
                        "understand",
                        "well",
                        "enough",
                        "to",
                        "explain",
                        "to",
                        "a",
                        "computer.",
                        "Art",
                        "is",
                        "everything",
                        "else",
                        "we",
                        "do",
                    ],
                    20,
                ),
                &[
                    "Science  is  what we",
                    "understand      well",
                    "enough to explain to",
                    "a  computer.  Art is",
                    "everything  else  we",
                    "do                  ",
                ],
            ),
        ];

        for ((words, max_width), expected) in test_cases {
            assert_eq!(
                S::full_justify(words.iter().copied().map(str::to_string).collect(), max_width),
                expected
            );
        }
    }
}
