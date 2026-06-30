pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_distinct(s: String) -> i32 {
        let mut seen = 0_u32;

        for c in s.bytes() {
            seen |= 1 << (c - b'a');
        }

        seen.count_ones().cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_distinct(s: String) -> i32 {
        Self::max_distinct(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
