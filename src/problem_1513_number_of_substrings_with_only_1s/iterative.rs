pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut result = 0;
        let mut length = 0_u32;

        for c in s.bytes() {
            if c == b'0' {
                length = 0;
            } else {
                length += 1;
                result += length;

                if result >= 1_000_000_007 {
                    result -= 1_000_000_007;
                }
            }
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_sub(s: String) -> i32 {
        Self::num_sub(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
