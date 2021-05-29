pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut pattern_to_word = HashMap::new();
        let mut word_to_pattern = HashMap::new();
        let mut pattern_iter = pattern.bytes();
        let mut word_iter = s.split(' ');

        loop {
            match (pattern_iter.next(), word_iter.next()) {
                (None, None) => break true,
                (Some(p), Some(word)) => {
                    if *pattern_to_word.entry(p).or_insert(word) != word
                        || *word_to_pattern.entry(word).or_insert(p) != p
                    {
                        return false;
                    }
                }
                _ => break false,
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn word_pattern(pattern: String, s: String) -> bool {
        Self::word_pattern(pattern, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
