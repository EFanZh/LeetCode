pub mod hash_set;

pub trait Solution {
    fn unique_morse_representations(words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&["gin", "zen", "gig", "msg"] as &[_], 2), (&["a"], 1)];

        for (words, expected) in test_cases {
            assert_eq!(
                S::unique_morse_representations(words.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
