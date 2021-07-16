pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let max_repeats = (a.len() * 2 + b.len() - 2) / a.len();
        let s = a.repeat(max_repeats);

        s.find(b.as_str())
            .map_or(-1, |i| ((i + b.len() + a.len() - 1) / a.len()) as _)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn repeated_string_match(a: String, b: String) -> i32 {
        Self::repeated_string_match(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
