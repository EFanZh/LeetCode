pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let mut iter = binary.bytes();
        let mut i = 0;

        loop {
            if let Some(c) = iter.next() {
                if c == b'0' {
                    break;
                }

                i += 1;
            } else {
                return binary;
            }
        }

        let first_zero = i;

        for c in iter {
            i += usize::from(c == b'0');
        }

        let mut bytes = binary.into_bytes();

        bytes[first_zero..].fill(b'1');
        bytes[i] = b'0';

        String::from_utf8(bytes).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_binary_string(binary: String) -> String {
        Self::maximum_binary_string(binary)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
