pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        const MODULUS: u64 = 1_000_000_007;

        let s = s.into_bytes();

        let mut cache_2 = 1;

        let mut cache_1 = match s.last().unwrap() {
            b'0' => 0,
            b'*' => 9,
            _ => 1,
        };

        for window in s.iter().zip(&s[1..]).rev() {
            let count = match window {
                (b'0', _) => 0,
                (b'1', b'0'..=b'9') | (b'2', b'0'..=b'6') => cache_1 + cache_2,
                (b'1', b'*') => cache_1 + cache_2 * 9,
                (b'2', b'7'..=b'9') | (b'3'..=b'9', _) => cache_1,
                (b'2', b'*') => cache_1 + cache_2 * 6,
                (b'*', b'0'..=b'6') => cache_1 * 9 + cache_2 * 2,
                (b'*', b'7'..=b'9') => cache_1 * 9 + cache_2,
                _ => cache_1 * 9 + cache_2 * 15,
            };

            cache_2 = mem::replace(&mut cache_1, count % MODULUS);
        }

        cache_1 as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_decodings(s: String) -> i32 {
        Self::num_decodings(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
