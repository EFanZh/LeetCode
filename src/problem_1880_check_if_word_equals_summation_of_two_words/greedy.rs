pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse(s: &str) -> u32 {
        let mut result = 0;

        for c in s.bytes() {
            result = result * 10 + u32::from(c) - u32::from(b'a');
        }

        result
    }

    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        Self::parse(&first_word) + Self::parse(&second_word) == Self::parse(&target_word)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        Self::is_sum_equal(first_word, second_word, target_word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
