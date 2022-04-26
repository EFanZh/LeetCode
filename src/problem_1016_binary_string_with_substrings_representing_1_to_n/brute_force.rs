pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str;

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        let n = n as u32;
        let mut buffer = [b'0'; 32];
        let mut start = 32;

        // Build buffer.

        {
            let mut x = n;

            while x != 0 {
                let offset = x.trailing_zeros() + 1;

                x >>= offset;
                start -= offset as usize;
                buffer[start] = b'1';
            }
        }

        // Check numbers in (n / 2, n] in reverse order.

        for _ in n / 2..n {
            if !s.contains(str::from_utf8(&buffer[start..]).unwrap()) {
                return false;
            }

            let mut i = 31;

            while let value @ b'0' = &mut buffer[i] {
                *value = b'1';
                i -= 1;
            }

            buffer[i] = b'0';

            if i == start {
                start += 1;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn query_string(s: String, n: i32) -> bool {
        Self::query_string(s, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
