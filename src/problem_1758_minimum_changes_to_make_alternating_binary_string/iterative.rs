pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut is_odd = false;
        let mut inverted = 0;

        for c in s.bytes() {
            inverted += u32::from(c & 1 == u8::from(is_odd));
            is_odd = !is_odd;
        }

        (s.len() as u32 - inverted).min(inverted) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(s: String) -> i32 {
        Self::min_operations(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
