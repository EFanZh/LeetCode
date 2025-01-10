pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut s = s;

        if let Some(mut i) = s.bytes().position(|c| c == b'*') {
            let mut buffer = s.into_bytes();
            let mut length = i - 1;

            loop {
                i += 1;

                if let Some(&c) = buffer.get(i) {
                    if c == b'*' {
                        length -= 1;
                    } else {
                        buffer[length] = c;
                        length += 1;
                    }
                } else {
                    break;
                }
            }

            buffer.truncate(length);

            s = String::from_utf8(buffer).unwrap();
        }

        s
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_stars(s: String) -> String {
        Self::remove_stars(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
