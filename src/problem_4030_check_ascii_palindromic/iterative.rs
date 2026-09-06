pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_palindromic(s: String) -> bool {
        let s = s.as_bytes();
        let n = s.len();
        let half = n.div_ceil(2);

        s[..half]
            .iter()
            .zip(s[n - half..].iter().rev())
            .all(|(&left, &right)| left.reverse_bits() == right)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_palindromic(s: String) -> bool {
        Self::is_palindromic(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
