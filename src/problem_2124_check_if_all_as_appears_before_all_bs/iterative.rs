pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_string(s: String) -> bool {
        let mut iter = s.bytes();

        while let Some(c) = iter.next() {
            if c == b'b' {
                return iter.all(|c| c == b'b');
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_string(s: String) -> bool {
        Self::check_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
