pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU8;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut s_counts = [0_u8; 26];

        for c in s.into_bytes() {
            s_counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let mut target_counts = [0_u8; 26];

        for c in target.into_bytes() {
            target_counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        let mut result = u8::MAX;

        for (&c_count, &target_count) in s_counts.iter().zip(&target_counts) {
            if let Some(target_count) = NonZeroU8::new(target_count) {
                result = result.min(c_count / target_count);
            }
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rearrange_characters(s: String, target: String) -> i32 {
        Self::rearrange_characters(s, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
