pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut result = 1;
        let mut prev = 0;

        for c in word.into_bytes() {
            result += i32::from(c == prev);
            prev = c;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn possible_string_count(word: String) -> i32 {
        Self::possible_string_count(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
