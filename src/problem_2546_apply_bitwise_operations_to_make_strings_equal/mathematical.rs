pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        s.as_bytes().contains(&b'1') == target.as_bytes().contains(&b'1')
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn make_strings_equal(s: String, target: String) -> bool {
        Self::make_strings_equal(s, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
