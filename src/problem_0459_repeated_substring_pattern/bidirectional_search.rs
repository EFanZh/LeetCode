pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut t = String::with_capacity(s.len() * 2 - 2);

        t.push_str(&s[1..]);
        t.push_str(&s[..s.len() - 1]);

        t.contains(&s)
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
