pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest_variance(s: String) -> i32 {
        let s = s.into_bytes();

        // Find used letters.

        let mut letters_bits = 0_u32;

        for &c in &s {
            letters_bits |= 1 << (c - b'a');

            if letters_bits == (1 << 26) - 1 {
                break;
            }
        }

        let mut letters = [0; 26];
        let mut letters_count = 0;

        while letters_bits != 0 {
            letters[letters_count] = b'a' + letters_bits.trailing_zeros() as u8;
            letters_count += 1;
            letters_bits &= letters_bits - 1;
        }

        // Prefix sums.

        let mut result = 0;
        let mut iter = letters[..letters_count].iter().copied();

        while let Some(lhs) = iter.next() {
            for rhs in iter.clone() {
                let mut diff = 0;
                let mut min_diff = i16::MAX;
                let mut candidate_min_diff = 0;
                let mut max_diff = i16::MIN;
                let mut candidate_max_diff = 0;

                for &c in &s {
                    if c == lhs {
                        diff += 1;
                        max_diff = candidate_max_diff;
                        candidate_max_diff = candidate_max_diff.max(diff);
                    } else if c == rhs {
                        diff -= 1;
                        min_diff = candidate_min_diff;
                        candidate_min_diff = candidate_min_diff.min(diff);
                    } else {
                        continue;
                    }

                    result = result.max((diff - min_diff).max(max_diff - diff));
                }
            }
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_variance(s: String) -> i32 {
        Self::largest_variance(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
