pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn decode(buffer: &mut Vec<u8>) {
        if let Some(mut i) = buffer.iter().position(|&c| c == b'#') {
            let mut decoded = i.saturating_sub(1);

            i += 1;

            while let Some(&c) = buffer.get(i) {
                if c == b'#' {
                    decoded = decoded.saturating_sub(1);
                } else {
                    buffer[decoded] = c;
                    decoded += 1;
                }

                i += 1;
            }

            buffer.truncate(decoded);
        }
    }

    pub fn backspace_compare(s: String, t: String) -> bool {
        let (mut s, mut t) = (s.into_bytes(), t.into_bytes());

        Self::decode(&mut s);
        Self::decode(&mut t);

        s == t
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn backspace_compare(s: String, t: String) -> bool {
        Self::backspace_compare(s, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
