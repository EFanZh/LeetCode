pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn count(word: &str) -> [u8; 26] {
        let mut result = [0; 26];

        for c in word.bytes() {
            result[usize::from(c) - usize::from(b'a')] += 1;
        }

        result
    }

    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut words = words;
        let mut prev_count = [0; 26];

        words.retain(|word| {
            let count = Self::count(word);
            let retain = count != prev_count;

            if retain {
                prev_count = count;
            }

            retain
        });

        words
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        Self::remove_anagrams(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
