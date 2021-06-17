pub mod cheating;

pub trait Solution {
    fn reverse_words(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("the sky is blue", "blue is sky the"),
            ("  hello world!  ", "world! hello"),
            ("a good   example", "example good a"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::reverse_words(s.to_string()), expected);
        }
    }
}
