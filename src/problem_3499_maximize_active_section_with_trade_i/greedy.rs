pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let mut iter = s.bytes();
        let mut ones = 0;

        // Find the first zero.

        loop {
            if let Some(c) = iter.next() {
                if c == b'0' {
                    break;
                }

                ones += 1;
            } else {
                return ones;
            }
        }

        // Count the length of the first chunk of zeroes.

        let mut prev_zeroes = 1;

        loop {
            if let Some(c) = iter.next() {
                if c == b'0' {
                    prev_zeroes += 1;
                } else {
                    break;
                }
            } else {
                return ones;
            }
        }

        // Iter through (chunk of ones, chunk of zeroes) chunks.

        let mut max_zero_pair_length = 0_u32;

        'outer: loop {
            ones += 1;

            let Some(c) = iter.next() else { break };

            if c == b'0' {
                // The chunk of ones ended, now iterate through the chunk of zeroes.

                let mut zeroes = 1;

                loop {
                    let Some(c) = iter.next() else {
                        max_zero_pair_length = max_zero_pair_length.max(prev_zeroes + zeroes);

                        break 'outer;
                    };

                    if c != b'0' {
                        // The chunk of zeroes ended.

                        max_zero_pair_length = max_zero_pair_length.max(prev_zeroes + zeroes);
                        prev_zeroes = zeroes;

                        // Now iterate through the chunk of ones.

                        break;
                    }

                    zeroes += 1;
                }
            }
        }

        ones + max_zero_pair_length.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_active_sections_after_trade(s: String) -> i32 {
        Self::max_active_sections_after_trade(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
