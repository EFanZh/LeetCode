pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn arrange_words(text: String) -> String {
        let mut text = text.into_bytes();

        text[0].make_ascii_lowercase();

        let mut words = text.split(|&c| c == b' ').collect::<Vec<_>>();

        words.sort_unstable_by_key(|word| (word.len(), word.as_ptr()));

        let mut result = words.join(&b' ');

        result[0].make_ascii_uppercase();

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn arrange_words(text: String) -> String {
        Self::arrange_words(text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
