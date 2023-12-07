pub mod iterative;

pub trait Solution {
    fn reverse_prefix(word: String, ch: char) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcdefd", 'd'), "dcbaefd"),
            (("xyxzxe", 'z'), "zxyxxe"),
            (("abcd", 'z'), "abcd"),
        ];

        for ((word, ch), expected) in test_cases {
            assert_eq!(S::reverse_prefix(word.to_string(), ch), expected);
        }
    }
}
