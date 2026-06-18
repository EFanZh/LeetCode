pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let mut s = s.into_bytes();

        s[..k.cast_unsigned() as usize].reverse();

        String::from_utf8(s).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reverse_prefix(s: String, k: i32) -> String {
        Self::reverse_prefix(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
