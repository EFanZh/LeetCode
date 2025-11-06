pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut extra = u64::from(n.cast_unsigned() - 1);
        let mut result = u64::from(x.cast_unsigned());
        let mut processed = 0;

        while extra != 0 {
            processed += (result >> processed).trailing_ones();

            let zeroes = (result >> processed).trailing_zeros();

            result |= (extra & u64::unbounded_shl(1, zeroes).wrapping_sub(1)) << processed;
            extra = u64::unbounded_shr(extra, zeroes);
            processed += zeroes;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_end(n: i32, x: i32) -> i64 {
        Self::min_end(n, x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
