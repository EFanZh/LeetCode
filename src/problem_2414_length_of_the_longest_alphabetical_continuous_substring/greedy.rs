pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let mut expected = 0;
        let mut length = 0_u32;
        let mut result = 0_u32;

        for c in s.into_bytes() {
            length = length * u32::from(c == expected) + 1;
            result = result.max(length);
            expected = c + 1;
        }

        result.max(length) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_continuous_substring(s: String) -> i32 {
        Self::longest_continuous_substring(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
