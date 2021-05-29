pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut result = 0;

        for c in s.bytes() {
            result *= 26;
            result += i32::from(c - (b'A' - 1));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn title_to_number(s: String) -> i32 {
        Self::title_to_number(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
