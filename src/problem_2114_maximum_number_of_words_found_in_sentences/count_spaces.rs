pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .iter()
            .map(|s| s.bytes().filter(|&c| c == b' ').count())
            .max()
            .unwrap() as i32
            + 1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn most_words_found(sentences: Vec<String>) -> i32 {
        Self::most_words_found(sentences)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
