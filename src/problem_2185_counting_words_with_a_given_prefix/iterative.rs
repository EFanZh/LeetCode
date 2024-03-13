pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let pref = pref.as_str();

        words.iter().filter(|word| word.starts_with(pref)).count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        Self::prefix_count(words, pref)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
