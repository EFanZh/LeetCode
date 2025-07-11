pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        words.iter().map(|word| word.as_bytes()[0]).eq(s.bytes())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_acronym(words: Vec<String>, s: String) -> bool {
        Self::is_acronym(words, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
