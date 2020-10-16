pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut pattern_indices = HashMap::new();
        let mut word_indices = HashMap::new();
        let mut pattern_iter = pattern.bytes();
        let mut word_iter = s.split(' ');
        let mut i = 0;

        loop {
            match (pattern_iter.next(), word_iter.next()) {
                (None, None) => break true,
                (Some(p), Some(word)) => {
                    if *pattern_indices.entry(p).or_insert(i) != *word_indices.entry(word).or_insert(i) {
                        return false;
                    }

                    i += 1;
                }
                _ => break false,
            }
        }
    }
}

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
