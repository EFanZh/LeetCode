pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    fn calculate_spaces_between_words(text: &[u8]) -> usize {
        let mut words = 0;
        let mut spaces = 0;
        let mut is_space = true;

        for &c in text {
            let new_is_space = c == b' ';

            spaces += usize::from(new_is_space);
            words += usize::from(is_space && !new_is_space);
            is_space = new_is_space;
        }

        if words < 2 { 0 } else { spaces / (words - 1) }
    }

    pub fn reorder_spaces(text: String) -> String {
        let text = text.as_bytes();
        let separator = iter::repeat(b' ').take(Self::calculate_spaces_between_words(text));
        let mut result = Vec::with_capacity(text.len());
        let mut word_start = None;

        for (i, &c) in text.iter().enumerate() {
            if c == b' ' {
                if let Some(start) = word_start {
                    word_start = None;

                    if !result.is_empty() {
                        result.extend(separator.clone());
                    }

                    result.extend(&text[start..i]);
                }
            } else if word_start.is_none() {
                word_start = Some(i);
            }
        }

        if let Some(start) = word_start {
            if !result.is_empty() {
                result.extend(separator);
            }

            result.extend(&text[start..]);
        }

        let missing = text.len() - result.len();

        result.extend(iter::repeat(b' ').take(missing));

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reorder_spaces(text: String) -> String {
        Self::reorder_spaces(text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
