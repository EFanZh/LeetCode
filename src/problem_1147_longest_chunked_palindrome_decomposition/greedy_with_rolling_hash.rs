pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(text: &[u8]) -> i32 {
        const D: u64 = 26;
        const MODULUS: u64 = 0x_09D8_9D89_D89D_89AD;

        let n = text.len();
        let mut iter = text.iter().copied().map(|c| u64::from(c - b'a'));
        let mut start = 0;
        let mut length = 0;
        let mut base_hash = 1;
        let mut left_hash = 0;
        let mut right_hash = 0;
        let mut result = 0;

        loop {
            if let Some(left) = iter.next() {
                if let Some(right) = iter.next_back() {
                    length += 1;
                    left_hash = (left_hash * D + left) % MODULUS;
                    right_hash = (base_hash * right + right_hash) % MODULUS;
                    base_hash = (base_hash * D) % MODULUS;

                    if left_hash == right_hash && text[start..start + length] == text[n - start - length..n - start] {
                        result += 2;

                        start += length;
                        length = 0;
                        base_hash = 1;
                        left_hash = 0;
                        right_hash = 0;
                    }
                } else {
                    result += 1;

                    break;
                }
            } else {
                if length != 0 {
                    result += 1;
                }

                break;
            }
        }

        result
    }

    pub fn longest_decomposition(text: String) -> i32 {
        Self::helper(text.as_bytes())
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_decomposition(text: String) -> i32 {
        Self::longest_decomposition(text)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
