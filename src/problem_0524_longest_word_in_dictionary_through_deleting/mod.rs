pub mod trie;

pub trait Solution {
    fn find_longest_word(s: String, dictionary: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abpcplea", &["ale", "apple", "monkey", "plea"] as &[_]), "apple"),
            (("abpcplea", &["a", "b", "c"]), "a"),
        ];

        for ((s, dictionary), expected) in test_cases {
            assert_eq!(
                S::find_longest_word(s.to_string(), dictionary.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
