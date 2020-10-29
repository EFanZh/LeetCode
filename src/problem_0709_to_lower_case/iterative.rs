pub struct Solution;

impl Solution {
    #[allow(clippy::clippy::wrong_self_convention)]
    pub fn to_lower_case(str: String) -> String {
        let mut s = str.into_bytes();

        for c in &mut s {
            if let b'A'..=b'Z' = c {
                *c += b'a' - b'A';
            }
        }

        String::from_utf8(s).unwrap()
    }
}

impl super::Solution for Solution {
    fn to_lower_case(str: String) -> String {
        Self::to_lower_case(str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
