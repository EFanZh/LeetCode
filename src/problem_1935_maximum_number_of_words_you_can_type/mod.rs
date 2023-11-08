pub mod iterative;

pub trait Solution {
    fn can_be_typed_words(text: String, broken_letters: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("hello world", "ad"), 1),
            (("leet code", "lt"), 1),
            (("leet code", "e"), 0),
        ];

        for ((text, broken_letters), expected) in test_cases {
            assert_eq!(
                S::can_be_typed_words(text.to_string(), broken_letters.to_string()),
                expected,
            );
        }
    }
}
