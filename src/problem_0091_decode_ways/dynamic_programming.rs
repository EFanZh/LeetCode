pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.into_bytes();

        s.last().map_or(1, |last| {
            let mut cache_2 = 1;
            let mut cache_1 = i32::from(*last != b'0');

            for window in s.windows(2).rev() {
                cache_2 = match window {
                    [b'0', _] => mem::replace(&mut cache_1, 0),
                    [b'1', _] | [b'2', b'0'..=b'6'] => {
                        let new_cache_1 = cache_1 + cache_2;

                        mem::replace(&mut cache_1, new_cache_1)
                    }
                    _ => cache_1,
                };
            }

            cache_1
        })
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
