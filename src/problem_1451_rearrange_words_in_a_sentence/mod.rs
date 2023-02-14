pub mod iterative;

pub trait Solution {
    fn arrange_words(text: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("Leetcode is cool", "Is cool leetcode"),
            ("Keep calm and code on", "On and keep calm code"),
            ("To be or not to be", "To be or to be not"),
        ];

        for (text, expected) in test_cases {
            assert_eq!(S::arrange_words(text.to_string()), expected);
        }
    }
}
