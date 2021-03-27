pub struct Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        let num = u32::from_le_bytes(num.to_le_bytes());

        let mut result = (0..32 - num.leading_zeros())
            .step_by(4)
            .rev()
            .map(|i| b"0123456789abcdef"[((num >> i) & 15) as usize])
            .collect::<Vec<_>>();

        if result.is_empty() {
            result.push(b'0');
        }

        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn to_hex(num: i32) -> String {
        Self::to_hex(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
