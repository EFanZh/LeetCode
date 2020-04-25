pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s = s.into_bytes();
        let mut stack = Vec::new();

        for byte in s.iter().copied() {
            match byte {
                b'(' | b'{' | b'[' => stack.push(byte),
                b')' => {
                    if stack.pop() != Some(b'(') {
                        return false;
                    }
                }
                b'}' => {
                    if stack.pop() != Some(b'{') {
                        return false;
                    }
                }
                b']' => {
                    if stack.pop() != Some(b'[') {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        stack.is_empty()
    }
}

impl super::Solution for Solution {
    fn is_valid(s: String) -> bool {
        Self::is_valid(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
