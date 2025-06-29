pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut result = Vec::new();

        for word in words {
            result.extend(
                word.split(separator)
                    .filter(|&word| !word.is_empty())
                    .map(str::to_string),
            );
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        Self::split_words_by_separator(words, separator)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
