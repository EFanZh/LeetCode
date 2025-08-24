pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_almost_equal_characters(word: String) -> i32 {
        let mut prev = 0;
        let mut result = 0;

        for c in word.bytes() {
            prev = if (c + 1).wrapping_sub(prev) < 3 {
                result += 1;

                0
            } else {
                c
            };
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_almost_equal_characters(word: String) -> i32 {
        Self::remove_almost_equal_characters(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
