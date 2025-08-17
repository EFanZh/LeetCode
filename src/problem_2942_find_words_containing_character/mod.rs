pub mod iterative;

pub trait Solution {
    fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["leet", "code"] as &[_], 'e'), &[0, 1] as &[_]),
            ((&["abc", "bcd", "aaaa", "cbc"], 'a'), &[0, 2]),
            ((&["abc", "bcd", "aaaa", "cbc"], 'z'), &[]),
        ];

        for ((words, x), expected) in test_cases {
            assert_eq!(
                S::find_words_containing(words.iter().copied().map(str::to_string).collect(), x),
                expected,
            );
        }
    }
}
