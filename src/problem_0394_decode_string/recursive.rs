pub struct Solution;

impl Solution {
    fn helper(iter: &mut impl Iterator<Item = u8>, result: &mut Vec<u8>) {
        while let Some(c) = iter.next() {
            match c {
                b'0'..=b'9' => {
                    let mut repeats = usize::from(c - b'0');

                    while let c @ b'0'..=b'9' = iter.next().unwrap() {
                        repeats = repeats * 10 + usize::from(c - b'0');
                    }

                    let saved_length = result.len();

                    Self::helper(iter, result);

                    let extra_space = (result.len() - saved_length) * (repeats - 1);

                    result.reserve(extra_space);

                    for i in 0..extra_space {
                        result.push(result[saved_length + i]);
                    }
                }
                b']' => break,
                _ => result.push(c),
            }
        }
    }

    pub fn decode_string(s: String) -> String {
        let mut result = Vec::new();

        Self::helper(&mut s.bytes(), &mut result);

        String::from_utf8(result).unwrap()
    }
}

impl super::Solution for Solution {
    fn decode_string(s: String) -> String {
        Self::decode_string(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
