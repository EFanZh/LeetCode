pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn make_iter(words: &[String]) -> impl Iterator<Item = u8> + '_ {
        words.iter().flat_map(|s| s.bytes())
    }

    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        Self::make_iter(&word1).eq(Self::make_iter(&word2))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        Self::array_strings_are_equal(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
