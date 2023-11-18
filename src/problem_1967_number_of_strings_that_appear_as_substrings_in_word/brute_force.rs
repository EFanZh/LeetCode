pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //
impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns.into_iter().filter(|p| word.contains(p)).count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        Self::num_of_strings(patterns, word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
