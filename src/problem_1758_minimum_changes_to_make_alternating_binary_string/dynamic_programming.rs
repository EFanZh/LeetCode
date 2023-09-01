pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut cache = (0, 0);

        for c in s.bytes() {
            cache = (
                cache.1 + u32::from(c) - u32::from(b'0'),
                cache.0 + u32::from(b'1') - u32::from(c),
            );
        }

        cache.0.min(cache.1) as _
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
