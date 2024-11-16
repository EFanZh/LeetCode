pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut mapping = [0_u8; 26];
        let mut next = b'a';

        for from in key.into_bytes() {
            if let Some(slot @ 0) = mapping.get_mut(usize::from(from).wrapping_sub(usize::from(b'a'))) {
                *slot = next;
                next += 1;

                if next > b'z' {
                    break;
                }
            }
        }

        let mut result = message.into_bytes();

        for from in &mut result {
            if let Some(&to) = mapping.get(usize::from(*from).wrapping_sub(usize::from(b'a'))) {
                *from = to;
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn decode_message(key: String, message: String) -> String {
        Self::decode_message(key, message)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
