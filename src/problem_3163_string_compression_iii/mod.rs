pub mod iterative;

pub trait Solution {
    fn compressed_string(word: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abcde", "1a1b1c1d1e"), ("aaaaaaaaaaaaaabb", "9a5a2b")];

        for (word, expected) in test_cases {
            assert_eq!(S::compressed_string(word.to_string()), expected);
        }
    }
}
