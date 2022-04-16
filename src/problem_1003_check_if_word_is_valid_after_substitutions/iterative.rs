pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let mut iter = s.bytes();
        let mut tail = 0;

        loop {
            match iter.next() {
                Some(b'a') => {
                    stack.push(tail);
                    tail = b'b';

                    continue;
                }
                value => {
                    if value.unwrap_or_default() != tail {
                        return false;
                    }
                }
            }

            // Continuation.

            match stack.last_mut() {
                None => return true,
                Some(frame) => {
                    if (*frame & 0x80) == 0 {
                        *frame |= 0x80;
                        tail = b'c';
                    } else {
                        tail = *frame & 0x7f;
                        stack.pop();
                    }
                }
            }
        }
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
