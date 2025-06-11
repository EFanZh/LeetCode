pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let mut prev = 0;
        let mut max_length_1 = 0;
        let mut max_length_2 = 0;

        s.bytes().fold(0_u32, |result, c| {
            (max_length_1, max_length_2) = if c == prev {
                (1, max_length_1 + 1)
            } else {
                (max_length_1 + 1, max_length_2 + 1)
            };

            prev = c;

            result.max(max_length_2)
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_semi_repetitive_substring(s: String) -> i32 {
        Self::longest_semi_repetitive_substring(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
