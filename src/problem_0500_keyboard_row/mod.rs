pub mod iterative;

pub trait Solution {
    fn find_words(words: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["Hello", "Alaska", "Dad", "Peace"] as &[_], &["Alaska", "Dad"] as &[_]),
            (&["omk"], &[]),
            (&["adsdf", "sfd"], &["adsdf", "sfd"]),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::find_words(words.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
