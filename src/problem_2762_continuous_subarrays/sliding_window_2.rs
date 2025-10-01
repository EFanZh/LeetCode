pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        const LENGTH_BITS: u32 = u64::BITS / 3;
        const LENGTH_MASK: u64 = (1 << LENGTH_BITS) - 1;
        const TOP_SHIFT: u32 = LENGTH_BITS * 2;
        const MIDDLE_SHIFT: u32 = LENGTH_BITS;
        const BOTTOM_SHIFT: u32 = 0;

        let mut lengths = 0_u64;
        let mut prev = 0;
        let mut result = 0;

        for num in nums {
            let diff = num - mem::replace(&mut prev, num);

            match diff {
                -2..=0 => lengths >>= LENGTH_BITS * (-diff).cast_unsigned(),
                1..=2 => {
                    lengths <<= LENGTH_BITS * diff.cast_unsigned();
                    lengths &= (1 << (LENGTH_BITS * 3)) - 1;
                }
                _ => lengths = 0,
            }

            lengths += (1 << TOP_SHIFT) | (1 << MIDDLE_SHIFT) | (1 << BOTTOM_SHIFT);

            let (top_length, middle_length, bottom_length) = (
                (lengths >> TOP_SHIFT) & LENGTH_MASK,
                (lengths >> MIDDLE_SHIFT) & LENGTH_MASK,
                (lengths >> BOTTOM_SHIFT) & LENGTH_MASK,
            );

            result += top_length.max(middle_length).max(bottom_length);
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        Self::continuous_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
