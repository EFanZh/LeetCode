pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn parse(s: &str) -> (u32, u32) {
        let mut non_repeating = 0_u32;
        let mut iter = s.bytes();

        loop {
            match iter.next() {
                Some(c @ b'0'..=b'9') => non_repeating = non_repeating * 10 + u32::from(c - b'0'),
                Some(b'.') => break,
                _ => return (non_repeating, 1),
            }
        }

        let mut non_repeating_base = 1;

        loop {
            match iter.next() {
                Some(c @ b'0'..=b'9') => {
                    non_repeating = non_repeating * 10 + u32::from(c - b'0');
                    non_repeating_base *= 10;
                }
                Some(b'(') => break,
                _ => return (non_repeating, non_repeating_base),
            }
        }

        let mut repeating = 0;
        let mut repeating_base = 1;

        while let Some(c @ b'0'..=b'9') = iter.next() {
            repeating = repeating * 10 + u32::from(c - b'0');
            repeating_base *= 10;
        }

        repeating_base -= 1;

        (
            non_repeating * repeating_base + repeating,
            non_repeating_base * repeating_base,
        )
    }

    pub fn is_rational_equal(s: String, t: String) -> bool {
        let lhs = Self::parse(&s);
        let rhs = Self::parse(&t);

        lhs.0 * rhs.1 == lhs.1 * rhs.0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_rational_equal(s: String, t: String) -> bool {
        Self::is_rational_equal(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
