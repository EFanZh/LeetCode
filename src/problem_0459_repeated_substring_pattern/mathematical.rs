pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut s = s;
        let n = s.len();

        s.extend_from_within(..n - 1);

        s[1..].contains(&s[..n])
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn repeated_substring_pattern(s: String) -> bool {
        Self::repeated_substring_pattern(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
