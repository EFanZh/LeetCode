pub struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s = s.as_bytes();
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

impl super::Solution for Solution {
    fn is_valid(s: String) -> bool {
        Self::is_valid(s)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
