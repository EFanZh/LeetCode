pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroI32;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut result = 0_u32;
        let mut bits = candidates.iter().fold(0, |bits, x| bits | x);

        while let Some(non_zero_bits) = NonZeroI32::new(bits) {
            let trailing_zeros = non_zero_bits.trailing_zeros();
            let probe = 1 << trailing_zeros;

            bits ^= probe;

            result = result.max(candidates.iter().fold(0, |count, &x| count + u32::from(x & probe != 0)));
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_combination(candidates: Vec<i32>) -> i32 {
        Self::largest_combination(candidates)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
