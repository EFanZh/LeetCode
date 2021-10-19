pub mod iterative;

pub trait Solution {
    fn expressive_words(s: String, words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("heeellooo", &["hello", "hi", "helo"] as &[_]), 1),
            (("zzzzzyyyyy", &["zzyy", "zy", "zyy"]), 3),
            (("heeellooo", &["heeelloooworld"]), 0),
            (("abcd", &["abc"]), 0),
            (("aaa", &["aaaa"]), 0),
        ];

        for ((s, words), expected) in test_cases {
            assert_eq!(
                S::expressive_words(s.to_string(), words.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
