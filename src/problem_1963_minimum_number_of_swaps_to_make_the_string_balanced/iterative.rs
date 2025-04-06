pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut stack = 0_u32;

        for c in s.into_bytes() {
            stack = stack.wrapping_add(if c == b'[' {
                1
            } else if stack == 0 {
                0
            } else {
                u32::MAX
            });
        }

        stack.div_ceil(2) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_swaps(s: String) -> i32 {
        Self::min_swaps(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
