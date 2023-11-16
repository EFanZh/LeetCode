pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut s = s.as_str();

        for word in words {
            if let Some(next_s) = s.strip_prefix(word.as_str()) {
                s = next_s;
            } else {
                return s.is_empty();
            }
        }

        s.is_empty()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        Self::is_prefix_string(s, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
