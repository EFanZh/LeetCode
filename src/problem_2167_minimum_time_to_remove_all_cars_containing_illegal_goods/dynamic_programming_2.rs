pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_time(s: String) -> i32 {
        let n = s.len() as u32;
        let mut left = n;
        let mut result = n;

        for c in s.into_bytes() {
            left = left.min(n) + (u32::from(c) - u32::from(b'0')) * 2 - 1;
            result = result.min(left);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_time(s: String) -> i32 {
        Self::minimum_time(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
