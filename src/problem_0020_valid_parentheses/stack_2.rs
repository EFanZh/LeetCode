pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s = s.into_bytes();
        let mut stack = Vec::new();

        for byte in s.iter().copied() {
            match byte {
                b'(' => stack.push(b')'),
                b'{' => stack.push(b'}'),
                b'[' => stack.push(b']'),
                _ => {
                    if stack.pop() != Some(byte) {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
