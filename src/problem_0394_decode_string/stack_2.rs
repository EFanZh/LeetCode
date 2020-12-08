pub struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut iter = s.bytes();

        while let Some(c) = iter.next() {
            match c {
                b'0'..=b'9' => {
                    let mut repeats = usize::from(c - b'0');

                    while let c @ b'0'..=b'9' = iter.next().unwrap() {
                        repeats = repeats * 10 + usize::from(c - b'0');
                    }

                    stack.push((repeats, result.len()));
                }
                b']' => {
                    if let Some((repeats, saved_length)) = stack.pop() {
                        let child_length = result.len() - saved_length;
                        let extra_space = child_length * (repeats - 1);

                        result.reserve(extra_space);

                        for i in 0..extra_space {
                            result.push(result[saved_length + i]);
                        }
                    } else {
                        break;
                    }
                }
                _ => result.push(c),
            }
        }

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
