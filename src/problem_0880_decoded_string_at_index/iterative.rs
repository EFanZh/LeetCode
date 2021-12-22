pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let s = s.as_bytes();
        let mut k = (k - 1) as u64;
        let mut cursor = 0;
        let mut tape_length = 0;

        while k != 0 {
            let c = s[cursor];

            if c.is_ascii_digit() {
                let extra_length = tape_length * u64::from(c - b'1');

                if k < extra_length {
                    k %= tape_length;
                    cursor = 0;
                    tape_length = 0;
                } else {
                    k -= extra_length;
                    cursor += 1;
                    tape_length += extra_length;
                }
            } else {
                k -= 1;
                cursor += 1;
                tape_length += 1;
            }
        }

        let result = s[cursor];

        char::from(if result.is_ascii_digit() { s[0] } else { result }).to_string()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn decode_at_index(s: String, k: i32) -> String {
        Self::decode_at_index(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
