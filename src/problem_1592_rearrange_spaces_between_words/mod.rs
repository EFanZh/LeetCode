pub mod iterative;

pub trait Solution {
    fn reorder_spaces(text: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("  this   is  a sentence ", "this   is   a   sentence"),
            (" practice   makes   perfect", "practice   makes   perfect "),
            ("a", "a"),
        ];

        for (text, expected) in test_cases {
            assert_eq!(S::reorder_spaces(text.to_string()), expected);
        }
    }
}
