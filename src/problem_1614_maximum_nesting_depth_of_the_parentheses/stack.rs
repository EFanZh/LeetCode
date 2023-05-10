pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut depth = 0_u32;
        let mut result = 0;

        for c in s.bytes() {
            match c {
                b'(' => {
                    depth += 1;
                    result = result.max(depth);
                }
                b')' => {
                    depth -= 1;
                }
                _ => {}
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_depth(s: String) -> i32 {
        Self::max_depth(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
