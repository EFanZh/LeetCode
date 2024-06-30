pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    fn parse_word(word: String) -> u32 {
        let mut result = 0;

        for c in word.into_bytes() {
            result |= 1 << (c - b'a');
        }

        result
    }

    pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
        let start_words = start_words.into_iter().map(Self::parse_word).collect::<HashSet<_>>();
        let mut result = 0;

        for word in target_words {
            let word = Self::parse_word(word);
            let mut bits = word;

            while bits != 0 {
                let bit = bits & bits.wrapping_neg();

                if start_words.contains(&(word ^ bit)) {
                    result += 1;

                    break;
                }

                bits ^= bit;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
        Self::word_count(start_words, target_words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
