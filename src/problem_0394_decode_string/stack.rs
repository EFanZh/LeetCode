pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut iter = s.bytes();

        loop {
            match iter.next() {
                None | Some(b']') => {
                    if let Some((repeats, saved_length)) = stack.pop() {
                        let extra_space = (result.len() - saved_length) * (repeats - 1);

                        result.reserve(extra_space);

                        for i in 0..extra_space {
                            result.push(result[saved_length + i]);
                        }
                    } else {
                        break;
                    }
                }
                Some(c @ b'0'..=b'9') => {
                    let mut repeats = usize::from(c - b'0');

                    while let c @ b'0'..=b'9' = iter.next().unwrap() {
                        repeats = repeats * 10 + usize::from(c - b'0');
                    }

                    stack.push((repeats, result.len()));
                }
                Some(c) => result.push(c),
            }
        }

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
