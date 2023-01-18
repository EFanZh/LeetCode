pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut words = words;
        let buffer = words.join(" ");

        words.retain(|word| buffer.matches(word).nth(1).is_some());

        words
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn string_matching(words: Vec<String>) -> Vec<String> {
        Self::string_matching(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
